use chrono::Utc;
use serde::Serialize;

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
