use serde::Serialize;

/// Simple HTTP Response structure for relaying status back to the requester.
#[derive(Serialize)]
pub struct SimpleResponse {
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
