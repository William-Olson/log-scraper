//! # New Relic Types Module
//!
//! A module for defining types used for interacting with New Relic API.
//!
//! ## Path
//!
//! new_relic/types.rs
//!
//! # Description
//!
//! Defines the response structures coming back from a New Relic API query.

use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct NewRelicLogItem {
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
    pub results: Vec<NewRelicLogItem>,
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

impl Clone for NewRelicLogItem {
    fn clone(&self) -> NewRelicLogItem {
        NewRelicLogItem {
            logger_name: self.logger_name.clone(),
            request_id: self.request_id.clone(),
            logtype: self.logtype.clone(),
            message: self.message.clone(),
            message_id: self.message_id.clone(),
            project: self.project.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}

impl ToString for NewRelicLogItem {
    fn to_string(&self) -> String {
        let s = serde_json::to_string(&self);
        s.unwrap_or("".to_owned())
    }
}
