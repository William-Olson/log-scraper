//! # Cron Tasks Module
//!
//! Handles starting and executing tasks via cron schedules.
//!
//! ## Path
//!
//! cron_tasks.rs
//!
//! # Description
//!
//! This module starts up a cron task to run the background tasks and functions.

use actix_web::web::Data;
use chrono::Utc;
extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler, Schedule};
use std::{io::Write, thread::JoinHandle, time::Duration};
use tracing::{event, instrument, Level};

use crate::{scraper, LogScraperState, env_config::{EnvConfig, LS_POLL_SCHEDULE}};

/// Starts up all the cron tasks and schedules for this module.
pub fn start(app_state: Data<LogScraperState>) -> JoinHandle<()> {
    // note: for building expressions, it's helpful to use: https://crontab.cronhub.io/

    let config_schedule = EnvConfig::global().get_val(LS_POLL_SCHEDULE);

    // // every 30 seconds, starting at 1 second past the minute
    // log_sync_task(app_state, "1/30 * * * * *".to_owned(), 300)

    // every 5 minutes, starting at 1 minute past the hour
    log_sync_task(app_state, config_schedule, 300)
}

/// Starts the cron task/schedule for synchronizing logs with the remote server.
#[instrument(name = "log_sync_task")]
fn log_sync_task(
    app_state: Data<LogScraperState>,
    cron_string: String,
    tick_rate_ms: u64,
) -> JoinHandle<()> {
    let tokio_handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        event!(
            Level::INFO,
            "starting cron thread now {}",
            Utc::now().to_rfc3339()
        );
        let mut scheduler = JobScheduler::new();
        let mut t: u64 = 0;
        let cron_schedule = cron_string.parse::<Schedule>().unwrap(); // every 30 seconds

        // start up cron task
        scheduler.add(Job::new(cron_schedule, move || {
            t += 1;
            let d = Utc::now().to_rfc3339();
            event!(
              Level::INFO,
                "Executing cron task #{t}  -  {d} (step-size = {tick_rate_ms} ms) :: (pattern = {cron_string})",
            );
            // run the scraper logic
            let data = app_state.clone();
            tokio_handle.block_on(async move {
                match scraper::run_sync(data).await {
                    Ok(_) => event!(Level::INFO, "Sync Complete!"),
                    Err(err) => event!(
                        Level::ERROR,
                        "An error occurred while running the sync: {err:?}"
                    ),
                };
            });
            std::io::stdout().flush().unwrap();
        }));

        loop {
            scheduler.tick();
            std::thread::sleep(Duration::from_millis(tick_rate_ms));
        }
    })
}
