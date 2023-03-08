//! # Api Types Module
//!
//! Defines API data models and helper methods.

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Response structure for relaying a status and message back to the requester.
#[derive(Serialize)]
pub struct SimpleResponse {
    pub ok: bool,
    pub message: String,
}

/// Response that contains a list of existing log filenames on the system.
#[derive(Serialize)]
pub struct LogListResponse {
    pub ok: bool,
    pub timestamp: String,
    pub log_files: Vec<String>,
}

/// Query parameters for requesting a paginated resource.
#[derive(Deserialize)]
pub struct PageParams {
    pub page: u32,
    pub page_size: u32,
}

/// Paginated resource response.
#[derive(Serialize)]
pub struct PagedLogContents {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub results: Vec<String>,
}

impl LogListResponse {
    /// Create a new LogListResponse with given logs.
    pub fn new(logs: Vec<String>) -> LogListResponse {
        LogListResponse {
            ok: true,
            log_files: logs,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl SimpleResponse {
    /// Creates a SimpleResponse struct with default values (ok=true, message="success").
    pub fn new() -> SimpleResponse {
        SimpleResponse {
            ok: true,
            message: "success".to_owned(),
        }
    }
    /// Creates a SimpleResponse struct with given status and message.
    pub fn from(ok_status: bool, resp_msg: &str) -> SimpleResponse {
        SimpleResponse {
            ok: ok_status,
            message: resp_msg.to_owned(),
        }
    }
}

/// A response structure that contains the current version of `log-scraper`.
#[derive(Serialize)]
pub struct VersionResponse {
    pub version: String,
}

impl VersionResponse {
    /// Creates a new VersionResponse struct from the given version.
    pub fn new(s: String) -> VersionResponse {
        VersionResponse { version: s }
    }
}
