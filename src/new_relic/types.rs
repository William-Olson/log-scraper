use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct NrqlResultItem {
    pub logger_name: String,
    pub request_id: String,
    pub logtype: String,
    pub message: String,
    pub message_id: String,
    pub project: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct NrqlResponsePayload {
    pub results: Vec<NrqlResultItem>,
}

#[derive(Deserialize, Serialize)]
pub struct NrqlResponseAccount {
    pub nrql: NrqlResponsePayload,
}

#[derive(Deserialize, Serialize)]
pub struct NrqlResponseActor {
    pub account: NrqlResponseAccount,
}

#[derive(Deserialize, Serialize)]
pub struct NrqlResponseData {
    pub actor: NrqlResponseActor,
}

#[derive(Deserialize, Serialize)]
pub struct NrqlResponse {
    pub data: NrqlResponseData,
}
