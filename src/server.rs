use actix_web::{get, post, HttpResponse, Responder};
use serde::Serialize;

use crate::scraper;

/// Simple HTTP Response structure for relaying status back to the requester.
#[derive(Serialize)]
struct SimpleResponse {
    pub ok: bool,
    pub message: String,
}
impl SimpleResponse {
    pub fn new() -> SimpleResponse {
        SimpleResponse {
            ok: true,
            message: "success".to_owned(),
        }
    }
}

/// Endpoint for checking health status.
/// Responds with `SimpleResponse` JSON.
#[get("/")]
pub async fn health_check_endpoint() -> impl Responder {
  let mut result = SimpleResponse::new();
  result.message = "Healthy and kicking!".to_owned();  
  HttpResponse::Ok().json(result)
}

/// Echo the body of a request back to the requester.
#[post("/echo")]
pub async fn echo_endpoint(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// Attempts to add logs to the filesystem from a remote server.
/// Fetches logs from remote server and saves them to disk.
#[get("/sync")]
pub async fn sync_logs_endpoint() -> impl Responder {
    scraper::attempt_sync().await;
    HttpResponse::Ok().json(SimpleResponse::new())
}
