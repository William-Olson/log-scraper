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
//! The `LogScraper` implementation was built as a workaround for the limitations of
//! log message retention policies provided by log aggregator services. Especially
//! those of free tier plans.
//!
//! ## Notes
//!
//! See the `server::sync_logs_endpoint` for fetching the latest logs from the remote server.
//! Health check endpoint for testing if API is live or not is here: `server::health_check_endpoint`.
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

use actix_web::{web, App, HttpServer, middleware::Logger};
use crate::env_config::{get_var_else, LS_SVC_PORT};

mod api;
mod caching;
mod env_config;
mod new_relic;
mod scraper;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // allow overriding the port via environment variable
    let default_port = "3333".to_owned();
    let port = get_var_else(LS_SVC_PORT, &default_port);

    // parse the port and start the server
    println!("Starting server on port {}", port);
    match port.parse::<u16>() {
        Ok(port_number) => {
            HttpServer::new(|| {
                App::new()
                    .wrap(Logger::new("%t %r (PID=%P) (IP=%a) %{User-Agent}i (time = %Ds)"))
                    .service(api::index_api::health_check_endpoint)
                    .service(api::index_api::echo_endpoint)
                    .service(api::index_api::version_endpoint)
                    .service(
                        web::scope("/logs")
                            .service(api::logs_api::sync_logs_endpoint)
                            .service(api::logs_api::get_log_list_endpoint)
                            .service(api::logs_api::get_log_contents_endpoint),
                    )
            })
            .bind(("0.0.0.0", port_number))?
            .run()
            .await
        }
        Err(err) => panic!("Error starting the server: {:?}", err),
    }
}
