//! # Logs Api module
//!
//! Provides endpoints for accessing and managing logs.
//!
//! ## get_log_list_endpoint
//!
//! Responds with a list of log files currently residing on the filesystem.
//!
//! GET `http://localhost:3333/logs/`
//!
//! ```
//! {
//!     "ok": true,
//!     "timestamp": "2023-01-02T00:00:00.000000+00:00",
//!     "log_files": [
//!       "app_2023-01-01.log",
//!       "app_2023-01-02.log"
//!     ]
//! }
//! ```
//!
//! ## get_log_contents_endpoint
//!
//! Attempts to read a log file's contents and return results
//!  in a paged response structure.
//!
//! GET `http://localhost:3333/logs/app_2023-01-01.log?page=1&page_size=100`
//!
//! ```
//! {
//!     "page": 1,
//!     "page_size": 100,
//!     "total": 2,
//!     "results": [
//!       "<file-contents-from-line-one>",
//!       "<file-contents-from-line-two>"
//!     ]
//! }
//! ```
//!
//! ## delete_log_endpoint
//!
//! Deletes the log file on disk and returns a success message.
//!
//! DELETE `http://localhost:3333/logs/app_2023-01-01.log`
//!
//! ```
//! {
//!     "message" : "Deleted app_2023-01-01.log successfully",
//!     "ok" : true
//! }
//! ```

use actix_web::{
    delete, get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use tracing::{event, instrument, Level};

use crate::{
    api::api_types::{LogListResponse, PageParams, PagedLogContents, SimpleResponse},
    scraper, storage, LogScraperState,
};

/// Attempts to add logs to the filesystem from a remote server.
/// Fetches logs from remote server and saves them to disk.
#[get("/sync")]
#[instrument(name = "sync_logs_endpoint")]
pub async fn sync_logs_endpoint(app_state: Data<LogScraperState>) -> impl Responder {
    match scraper::run_sync(app_state).await {
        Ok(_) => event!(Level::INFO, "Sync Complete!"),
        Err(err) => event!(
            Level::ERROR,
            "An error occurred while running the sync: {err:?}"
        ),
    };
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
#[instrument(name = "get_log_contents_endpoint")]
pub async fn get_log_contents_endpoint(
    paging: Query<PageParams>,
    id: Path<String>,
) -> impl Responder {
    event!(Level::INFO, "Looking for file with name {id}");

    // sanitize id so they can't traverse directories
    let sanitized = id.replace("/", "");
    let page = paging.page.unwrap_or(1);
    let page_size = paging.page_size.unwrap_or(100);

    // sanity check on page and page_size
    if page == 0 {
        return HttpResponse::BadRequest().json(SimpleResponse::from(
            false,
            "Invalid value for page parameter.",
        ));
    }
    if page_size == 0 {
        return HttpResponse::BadRequest().json(SimpleResponse::from(
            false,
            "Invalid value for page_size parameter.",
        ));
    }

    // validate file exists
    if !storage::has_file(&sanitized) {
        event!(Level::ERROR, "Unable to find file with name {sanitized}");
        return HttpResponse::NotFound().json(SimpleResponse::from(false, "Unable to find file"));
    }

    // read the contents of the file
    match storage::total_lines(&sanitized).await {
        Ok(total) => {
            let lines = storage::get_lines_by_page(&sanitized, page, page_size).await;
            HttpResponse::Ok().json(PagedLogContents {
                total: total.try_into().unwrap_or(0),
                results: lines,
                page,
                page_size,
            })
        }
        Err(err) => {
            event!(
                Level::ERROR,
                "An error occurred reading lines from file {sanitized} \n{err:?}"
            );
            HttpResponse::InternalServerError().json(SimpleResponse::from(
                false,
                "Error occurred while reading the file",
            ))
        }
    }
}

/// Attempts to delete a log file.
#[delete("/{id}")]
#[instrument(name = "delete_log_endpoint")]
pub async fn delete_log_endpoint(id: Path<String>) -> impl Responder {
    event!(Level::INFO, "Deleting log file with name {id}");
    let sanitized = id.replace("/", "");

    // validate file exists
    if !storage::has_file(&sanitized) {
        event!(Level::ERROR, "Unable to find file with name {sanitized}");
        return HttpResponse::NotFound().json(SimpleResponse::from(false, "Unable to find file"));
    }

    match storage::delete_file(&sanitized).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse::from(
            true,
            &format!("Deleted {id} successfully"),
        )),
        Err(err) => {
            event!(Level::ERROR, "Unable to delete file {sanitized} \n{err:?}");
            HttpResponse::InternalServerError().json(SimpleResponse::from(
                false,
                "Error occurred while deleting the file",
            ))
        }
    }
}
