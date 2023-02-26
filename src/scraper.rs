use crate::{caching, new_relic::NewRelic};

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
    let log_results = nr.get_logs(&last_seen).await;

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
