
use actix_web::{get, delete, HttpResponse, Responder, web::{Path, Query}};

use crate::{scraper, api::api_types::{SimpleResponse, LogListResponse, PageParams, PagedLogContents}, storage};

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
pub async fn get_log_contents_endpoint(paging: Query<PageParams>, id: Path<String>) -> impl Responder {
    println!("Looking for file with name {}", id);

    // parse the query params for page and page_size
    if paging.page_size <= 0 {
        return HttpResponse::BadRequest().json(SimpleResponse::from(false, "Error parsing page_size parameter."));
    }
    if paging.page <= 0 {
        return HttpResponse::BadRequest().json(SimpleResponse::from(false, "Error parsing page parameter."));
    }

    // validate file exists
    if !storage::has_file(&id) {
        return HttpResponse::NotFound().json(SimpleResponse::from(false, &format!("Unable to find file with name {}", id)));
    }

    // read the contents of the file
    match storage::total_lines(&id).await {
        Ok(total) => {
            let lines = storage::get_lines_by_page(&id, paging.page, paging.page_size).await;
            HttpResponse::Ok().json(PagedLogContents{
                total: total.try_into().unwrap_or(0),
                results: lines,
                page: paging.page,
                page_size: paging.page_size
            })
        },
        Err(err) => {
            println!("Warning: error occurred reading lines from file {} \n{:?}", &id, err);
            HttpResponse::InternalServerError().json(SimpleResponse::from(false, "Error occurred while reading file"))
        },
    }

}

/// Attempts to delete a log file.
#[delete("/{id}")]
pub async fn delete_log_endpoint(id: Path<String>) -> impl Responder {
    // TODO: implement this
    println!("Deleting log file with name {}", id);
    HttpResponse::Ok().json(SimpleResponse::new())
}
