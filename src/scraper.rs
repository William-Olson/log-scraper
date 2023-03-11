//! # Scraper Module
//!
//! Attempts to sync and persist logs found on the remote server.

use actix_web::web::Data;

use crate::{caching, new_relic::NewRelic, storage, LogScraperState};
use tracing::{info, instrument, warn};

/// Saves the string using the caching module. Fails softly
/// with error message printed to stdout.
#[instrument(name = "save_to_cache")]
async fn save_to_cache(val: String) {
    match caching::set_cached_val(val).await {
        Ok(()) => info!("Success: saved cached value successfully."),
        Err(err) => {
            warn!("Warning: An error occurred saving to cache: {:?}", err)
        }
    };
}

/// Runs the sync with thread safe caching of timestamp via LogScraperState.
#[instrument(name = "run_sync")]
pub async fn run_sync(data: Data<LogScraperState>) -> tokio::io::Result<()> {
    // acquire lock on mutex
    let mut last_seen = data.last_seen.lock().unwrap();
    let t: String = (*last_seen.clone()).to_string();

    // run sync operation
    info!("Sending value to log_scraper: {}", t);
    let u = attempt_sync(t).await;

    // update the underlying mutex value
    *last_seen = u.clone();
    info!("Updated LogScraperState with last_seen: {}", u);
    Ok(())
}

/// Attempts to sync local logs from the remote log service and saves them to disk.
/// Relies on the timestamp of the last seen log. This timestamp is stored as a String
/// and can be passed in as a parameter or is read from the remote cache using the
/// caching module.
#[instrument(name = "attempt_sync")]
async fn attempt_sync(timestamp_from_memory: String) -> String {
    // only hit the cache if needed for reading
    let last_seen = if !timestamp_from_memory.is_empty() {
        info!("Using value of last_seen from memory: {timestamp_from_memory}");
        timestamp_from_memory
    } else {
        info!("Reading from remote cache...");
        match caching::get_cached_val().await {
            Ok(last_seen) => {
                info!("Found value from cache: {last_seen}");
                last_seen.to_string()
            }
            Err(err) => {
                warn!("Warning: An error occurred reading from cache: {err:?}");
                "".to_owned()
            }
        }
    };

    let nr = NewRelic::new();
    let log_results = nr.logs_since(&last_seen).await;

    // bail if there are no new logs to sync
    if log_results.len() == 0 {
        // but make sure we cache the value to stay in sync
        info!("No logs found. Caching old timestamp to remote: {last_seen}");
        save_to_cache(last_seen.clone()).await;
        return last_seen;
    }

    // print the logs to the console
    nr.print_logs(&log_results);

    let latest_log = nr.find_latest(&log_results);
    let watermark = nr.to_watermark(&latest_log);

    // Save the logs to disk
    let filename = storage::get_filename(latest_log.timestamp).await;
    info!("Writing to file: {filename} . . .");
    let data = log_results
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let success = match storage::write_to_file(&filename, &data.join("\n")).await {
        Ok(()) => {
            info!("Successfully wrote logs to {filename} √");
            let line_count = storage::total_lines(&filename).await.unwrap_or(0);
            info!("Total lines in file: {line_count}");
            true
        }
        Err(err) => {
            warn!("Warning: An error occurred saving logs to file: {err:?}");
            false
        }
    };

    if !success {
        // revert to previous since write wasn't successful
        return last_seen;
    }

    info!("Caching last_seen timestamp on remote: {watermark}");
    save_to_cache(watermark.clone()).await;

    // return the updated timestamp for saving to memory
    watermark
}
