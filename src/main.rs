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
//! - `NRLS_ACCOUNT_ID`: New Relic Account ID
//! - `NRLS_API_KEY`: New Relic API Key
//! - `REDIS_URL`: Redis URL with port
//! - `LS_SVC_PORT`: (optional) App server port (defaults to `3333`)

use std::env;
use actix_web::{App, HttpServer, web};

mod caching;
mod new_relic;
// mod storage;
mod scraper;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // allow overriding the port via environment variable
    let default_port = "3333".to_owned();
    let port = {
        match env::var("LS_SVC_PORT") {
            Ok(t) => t,
            Err(_) => default_port.clone(),
        }
    };

    // parse the port and start the server
    println!("Starting server on port {}", port);
    match port.parse::<u16>() {
        Ok(port_number) => {
            HttpServer::new(|| {
                App::new()
                    .service(server::health_check_endpoint)
                    .service(server::echo_endpoint)
                    .service(
                        web::scope("/logs")
                            .service(server::sync_logs_endpoint)
                    )
            })
            .bind(("0.0.0.0", port_number))?
            .run()
            .await
        }
        Err(err) => panic!("Error starting the server: {:?}", err),
    }
}
