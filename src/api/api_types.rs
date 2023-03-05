use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Simple HTTP Response structure for relaying status back to the requester.
#[derive(Serialize)]
pub struct SimpleResponse {
    pub ok: bool,
    pub message: String,
}

/// Simple HTTP Response structure for relaying status back to the requester.
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
    pub fn new(logs: Vec<String>) -> LogListResponse {
        LogListResponse {
            ok: true,
            log_files: logs,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl SimpleResponse {
    pub fn new() -> SimpleResponse {
        SimpleResponse {
            ok: true,
            message: "success".to_owned(),
        }
    }
    pub fn from(ok_status: bool, resp_msg: &str) -> SimpleResponse {
        SimpleResponse {
            ok: ok_status,
            message: resp_msg.to_owned(),
        }
    }
}

/// Simple HTTP Response structure for relaying status back to the requester.
#[derive(Serialize)]
pub struct VersionResponse {
    pub version: String,
}

impl VersionResponse {
    pub fn new(s: String) -> VersionResponse {
        VersionResponse { version: s }
    }
}
