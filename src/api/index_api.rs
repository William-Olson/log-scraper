//! # Index Api Module
//!
//! Provides useful root endpoints such as health (liveness) check endpoint
//! and a version information endpoint etc.

use actix_web::{get, post, HttpResponse, Responder};
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

use crate::api::api_types::{SimpleResponse, VersionResponse};

/// Endpoint for checking health status.
/// Responds with `SimpleResponse` JSON.
#[get("/")]
pub async fn health_check_endpoint() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse::from(true, "Healthy and kicking!"))
}

/// Echo the body of a request back to the requester.
#[post("/echo")]
pub async fn echo_endpoint(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// Echo the body of a request back to the requester.
#[get("/version")]
pub async fn version_endpoint() -> impl Responder {
    let ls_version = VERSION.unwrap_or("(NA)");
    HttpResponse::Ok().json(VersionResponse::new(ls_version.to_owned()))
}
