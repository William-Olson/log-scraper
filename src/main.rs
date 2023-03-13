//! # Log Scraper
//!
//! Handles fetching, persisting, and managing logs sourced from a third party service.
//!
//! # Description
//!
//! Starts an HTTP server and provides endpoints for pulling down, saving, and managing log messages.
//!
//! ### Purpose
//!
//! This service implementation was built as a workaround for the limitations of
//! log message retention policies provided by log aggregator services. Especially
//! those of free tier plans.
//!
//! ## Notes
//!
//! See the `api::logs_api::sync_logs_endpoint` for fetching the latest logs from the remote server.
//! Health check endpoint for testing if API is live or not is here: `api::index_api::health_check_endpoint`.
//!
//! ## Environment Variables
//!
//! There are multiple important environment variables that need to be configured.
//! Critical ones listed below but see the `env_config` module for full list of
//! available and required ones.
//!
//! - `NRLS_ACCOUNT_ID`: New Relic Account ID
//! - `NRLS_API_KEY`: New Relic API Key
//! - `REDIS_URL`: Redis URL with port
//! - `LS_SVC_PORT`: (optional) App server port (defaults to `3333`)

use crate::env_config::{EnvConfig, CONFIG, LOG_DIRECTORY, LS_SVC_PORT};
use actix_files as fs;
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};
use tokio::sync::Mutex;
use tracing::{event, instrument, Level};
use tracing_subscriber::fmt::format;

mod api;
mod caching;
mod cron_tasks;
mod env_config;
mod new_relic;
mod scraper;
mod storage;

// use tokio::lock::Mutex;

#[derive(Debug)]
pub struct LogScraperState {
    last_seen: Mutex<String>, // last seen log timestamp in milliseconds
}

#[actix_web::main]
#[instrument(name = "log_scraper")]
async fn main() -> std::io::Result<()> {
    // register a log subscriber to print events & messages to stdout
    tracing_subscriber::fmt()
        .compact()
        // exclude fields (except message) for now
        .fmt_fields(format::debug_fn(|writer, field, field_data| {
            if field.to_string() != "message" {
                return Ok(());
            }
            write!(writer, "{field_data:?}")
        }))
        // .with_file(true)
        // .with_line_number(true)
        // .with_thread_ids(true)
        // .with_target(true)
        .init();

    // initialize the environment config
    match CONFIG.set(EnvConfig::new()) {
        Ok(_) => event!(Level::INFO, "Loaded config successfully"),
        Err(_) => panic!("Error loading config!"),
    }

    // setup our logging storage area
    storage::ensure_log_directory().await?;

    // create our app state
    let app_state: Data<LogScraperState> = Data::new(LogScraperState {
        last_seen: Mutex::new("".to_owned()),
    });

    // start up cron jobs
    cron_tasks::start(app_state.clone());

    // get server port from environment variables or defaults
    let port = EnvConfig::global().get_val(LS_SVC_PORT);

    // include some basic info for server logging
    let api_logger_pattern = "%t %r (IP=%a) %{User-Agent}i (time = %D ms)";

    // parse the port and start the server
    event!(Level::INFO, "Starting server on port {port}");
    match port.parse::<u16>() {
        Ok(port_number) => {
            HttpServer::new(move || {
                App::new()
                    // don't attach logger to this scope to reduce noise from health checks
                    .service(api::index_api::health_check_endpoint)
                    .service(api::index_api::version_endpoint)
                    .service(
                        web::scope("/files")
                            .wrap(Logger::new(api_logger_pattern))
                            .service(
                                // allow viewing log files directly
                                fs::Files::new("/", EnvConfig::global().get_val(LOG_DIRECTORY))
                                    .show_files_listing(),
                            ),
                    )
                    // // Note: to generate docs...
                    // // run the following command in the terminal:
                    // //  `cargo doc --document-private-items && mv ./target/doc ./docs`
                    // .service(
                    //     web::scope("/docs")
                    //         .wrap(Logger::new(api_logger_pattern))
                    //         .service(
                    //             // allow viewing log files directly
                    //             fs::Files::new("/", "./docs").show_files_listing(),
                    //         ),
                    // )
                    .service(
                        web::scope("/logs")
                            .app_data(app_state.clone())
                            .wrap(Logger::new(api_logger_pattern))
                            .service(api::logs_api::sync_logs_endpoint)
                            .service(api::logs_api::get_log_list_endpoint)
                            .service(api::logs_api::delete_log_endpoint)
                            .service(api::logs_api::get_log_contents_endpoint),
                    )
            })
            .bind(("0.0.0.0", port_number))?
            .run()
            .await
        }
        Err(err) => panic!("Error starting the server: {err:?}"),
    }
}
