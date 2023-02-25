
//! # New Relic
//! 
//! A module for interacting with New Relic's API.
//!
//! ## Path
//!
//! new_relic/mod.rs
//!
//! # Description
//!
//! Allows fetching logs from New Relic Graph QL API

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use reqwest::{header::{HeaderMap, HeaderValue} };
use chrono::{serde::ts_milliseconds, DateTime, Utc};

use crate::env_config::{get_var_else, NRLS_ACCOUNT_ID, NRLS_API_KEY};


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


/// Creates a New Relic Graphql Request Payload with the given Account
/// ID and simple NRQL expression.
///
/// **account_id**: The account New Relic Account ID
/// **query**: The NRQL Query
///
/// # Examples
///
/// ```
/// create_nrql_payload("1234567", "SELECT * FROM Log SINCE 7 DAYS AGO");
/// create_nrql_payload("1234567", "SELECT * FROM Log SINCE 1 HOURS AGO");
/// // => "{ actor { account(id: 1234567) { nrql(query: \"SELECT * FROM Log SINCE 1 HOURS AGO\") { results } } } }"
/// ```
fn create_nrql_payload(account_id: &str, query: &str) -> String {
  let mut result_str: String = "".to_owned();
  result_str.push_str("{ actor { account(id: ");
  result_str.push_str(account_id);
  result_str.push_str(") { nrql(query: \"");
  result_str.push_str(query);
  result_str.push_str("\") { results } } } }");
  result_str
}


pub struct NewRelic {}

impl NewRelic {
    /// Creates a new `NewRelic` struct.
    pub fn new() -> NewRelic {
        let new_instance = NewRelic {};
        new_instance
    }

    /// Fetches logs from `api.newrelic.com/graphiql`.
    ///
    /// Requires Account ID (`NRLS_ACCOUNT_ID`) and API key 
    /// (`NRLS_API_KEY`) to be set via environment variables.
    pub async fn get_logs(&self) -> NrqlResponse {
      println!("... ** Fetching logs ** ...");

      // use environment variables if present, else fallback to undefined string
      let fallback_value = "undefined".to_owned();
      let nrls_id = get_var_else(NRLS_ACCOUNT_ID, &fallback_value.clone());
      let nrls_key = get_var_else(NRLS_API_KEY, &fallback_value.clone());

      // construct request body with the graphql query
      let query: String = create_nrql_payload(&nrls_id, "SELECT * FROM Log SINCE 3 DAYS AGO");
      println!("Constructed query: {}", query);

      // set api key in headers
      assert!(!nrls_key.is_empty(), "API Header Key is Missing!");
      let mut headers = HeaderMap::new();
      let k = nrls_key.as_str();
      headers.append(
          "API-Key",
          HeaderValue::from_str(k).expect("Header key is missing"),
      );

      let client = reqwest::Client::new();
      let response = client
          .get("https://api.newrelic.com/graphql")
          .headers(headers)
          .body(query)
          .send();

      match response.await {
          Ok(res) => {
              let body_text = res.text().await.expect("Failed retrieving body");
      
              // return match res.json::<NrqlResponse>().await {
              return match serde_json::from_str::<NrqlResponse>(&body_text) {
                  Ok(j) => j,
                  Err(ei) => panic!("Error parsing response: \n{:?} \n{:?}", body_text, ei),
              }
          }
          Err(e) => panic!("Error sending request: {:?}", e),
      };
  }

  /// Helper for printing logs to the console.
  pub fn print_logs(&self, log_response: NrqlResponse) {
    println!("Logs: \n-------\n");
    let mut logs: Vec<NrqlResultItem> = log_response.data.actor.account.nrql.results;
    logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    logs.iter()
        .for_each(|row| println!("{}", to_string(row).unwrap()));
    println!("\n\n");
  }

  /// Get the timestamp in milliseconds of the NrqlResultItem to use as a watermark
  pub fn to_watermark(&self, r: &NrqlResultItem) -> String {
    format!("{}", r.timestamp.timestamp_millis())
  }

}

