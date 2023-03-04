
use actix_web::{get, delete, HttpResponse, Responder};
use chrono::Utc;

use crate::{scraper, api::api_types::{SimpleResponse, LogListResponse}, storage};

/// Attempts to add logs to the filesystem from a remote server.
/// Fetches logs from remote server and saves them to disk.
#[get("/sync")]
pub async fn sync_logs_endpoint() -> impl Responder {
    scraper::attempt_sync().await;
    HttpResponse::Ok().json(SimpleResponse::new())
}

/// Attempts to read the list of log filenames on disk and
/// returns them.
#[get("/")]
pub async fn get_log_list_endpoint() -> impl Responder {
    let resp = storage::get_log_filenames().await;
    HttpResponse::Ok().json(LogListResponse::new(resp))
}

/// Attempts to read a log file's contents and return results
/// in a paged response structure.
#[get("/{id}")]
pub async fn get_log_contents_endpoint() -> impl Responder {
    // TODO: implement this with pagination
    HttpResponse::Ok().json(SimpleResponse::new())
}

/// Attempts to delete a log file.
#[delete("/{id}")]
pub async fn delete_log_endpoint() -> impl Responder {
    // TODO: implement this
    HttpResponse::Ok().json(SimpleResponse::new())
}
