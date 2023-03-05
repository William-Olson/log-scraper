use crate::{caching, new_relic::NewRelic, storage};

/// Attempts to sync local logs from the remote log service
pub async fn attempt_sync() {
    let last_seen = match caching::get_cached_val().await {
      Ok(last_seen) => last_seen.to_string(),
      Err(err) => {
          println!("Warning: An error occurred reading from cache: {:?}", err);
          "".to_owned()
      }
    };

    let nr = NewRelic::new();
    let log_results = nr.logs_since(&last_seen).await;

    if log_results.len() > 0 {
        nr.print_logs(&log_results);

        let latest_log = nr.find_latest(&log_results);
        let watermark = nr.to_watermark(&latest_log);

        // Save the logs to disk
        let filename = storage::get_filename(latest_log.timestamp).await;
        println!("Writing to file: {} . . .", filename);
        let data = log_results.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        match storage::write_to_file(&filename, &data.join("\n")).await {
          Ok(()) => {
            println!("Successfully wrote logs to {} √", filename);
            let line_count = storage::total_lines(&filename).await.unwrap_or(0);
            print!("Total lines in file: {}", line_count)
          },
          Err(err) => println!("Warning: An error occurred saving logs to file: {:?}", err),
        }

        println!("setting last_seen to timestamp: {}", watermark);
        match caching::set_cached_val(watermark).await {
            Ok(()) => println!("Success: saved cached value successfully."),
            Err(err) => {
              println!("Warning: An error occurred saving to cache: {:?}", err)
            },
        }
    }

}
