use crate::{caching, new_relic::NewRelic};

async fn run_log_sync(last_seen: &str) {
    // if found, we want to use it for our query for fetching logs
    if last_seen.len() > 0 {
        println!("found last_seen value in cache: {:?}", last_seen);
        // TODO: handle last_seen date from cache flow
        // return;
    }

    // otherwise fetch logs from a sensible date and cache last_seen
    println!("last_seen is empty. fetching reords now...");
    let nr = NewRelic::new();

    // TODO: specify since date for query
    let log_results = nr.get_logs().await;

    if log_results.data.actor.account.nrql.results.len() > 0 {
        let first_row = log_results.data.actor.account.nrql.results.get(0).unwrap();
        let watermark = nr.to_watermark(first_row);
        println!("setting last_seen to timestamp: {}", watermark);
        match caching::set_cached_val(watermark).await {
            Ok(()) => println!("Success: saved cached value successfully."),
            Err(err) => {
              println!("Warning: An error occurred saving to cache: {:?}", err)
            },
        }
    }

    nr.print_logs(log_results);
}

/// Attempts to sync local logs from the remote log service
pub async fn attempt_sync() {
    // check last_seen cached value
    match caching::get_cached_val().await {
        Ok(last_seen) => run_log_sync(&last_seen).await,
        Err(err) => {
            println!("Warning: An error occurred reading from cache: {:?}", err);
            run_log_sync("").await
        }
    }
}
