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

mod types;

use crate::env_config::{EnvConfig, NRLS_ACCOUNT_ID, NRLS_API_KEY};
use crate::new_relic::types::{NewRelicLogItem, NrqlResponse};
use chrono::Duration;
use reqwest::header::{HeaderMap, HeaderValue};
use tracing::{event, instrument, trace, warn, Level};

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

#[derive(Debug)]
pub struct NewRelic {}

impl NewRelic {
    /// Creates a new `NewRelic` struct.
    pub fn new() -> NewRelic {
        NewRelic {}
    }

    /// Fetches logs from `api.newrelic.com/graphiql`.
    ///
    /// Requires Account ID (`NRLS_ACCOUNT_ID`) and API key
    /// (`NRLS_API_KEY`) to be set via environment variables.
    #[instrument(name = "logs_since")]
    pub async fn logs_since(&self, timestamp: &str) -> Result<Vec<NewRelicLogItem>, String> {
        // fetch new relic logs since last timestamp
        let resp = self.get_logs(timestamp).await?;

        // don't return a super nested structure like the response is, just grab results
        let mut logs = resp.data.actor.account.nrql.results;

        if logs.is_empty() {
            return Ok(logs);
        }

        // ensure logs are sorted by timestamp
        logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(logs)
    }

    // Makes an http call to fetch logs from New Relic API.
    #[instrument(name = "get_logs")]
    async fn get_logs(&self, timestamp_millis: &str) -> Result<NrqlResponse, String> {
        trace!("... ** Fetching logs ** ...");
        let env = EnvConfig::global();
        let nrls_id = env.get_val(NRLS_ACCOUNT_ID);
        let nrls_key = env.get_val(NRLS_API_KEY);

        // construct request payload with the graphql query
        // fallback to 7 days ago if timestamp not found
        let since: String = if timestamp_millis.is_empty() {
            "7 DAYS AGO".to_owned()
        } else {
            timestamp_millis.to_owned()
        };

        let log_query = format!("SELECT * FROM Log SINCE {since}");
        let nrql_payload: String = create_nrql_payload(&nrls_id, &log_query);
        trace!("Constructed query: {nrql_payload}");

        // set api key in headers
        if nrls_key.is_empty() {
            return Err("No New Relic key provided!".to_owned());
        }
        let mut headers = HeaderMap::new();
        headers.append(
            "API-Key",
            HeaderValue::from_str(&nrls_key).expect("Header key is missing"),
        );

        let client = reqwest::Client::new();
        let request = client
            .get("https://api.newrelic.com/graphql")
            .headers(headers)
            .body(nrql_payload)
            .send();

        let response = match request.await {
            Ok(resp) => resp,
            Err(err) => {
                event!(Level::ERROR, "{err:?}");
                return Err("Failed to request data from the remote server".to_owned());
            }
        };

        let response_body = match response.text().await {
            Ok(resp_result) => resp_result,
            Err(parse_err) => {
                event!(Level::ERROR, "{parse_err:?}");
                return Err("Failed to parse response data from remote server".to_owned());
            }
        };

        match serde_json::from_str::<NrqlResponse>(&response_body) {
            Ok(j) => Ok(j),
            Err(e) => {
                warn!("{e:?}");
                Err("Error fetching logs".to_owned())
            }
        }
    }

    /// Helper for determining the log item with the latest timestamp in a list.
    /// Caution: panics! if the list is empty.
    pub fn find_latest(&self, logs: &Vec<NewRelicLogItem>) -> NewRelicLogItem {
        if logs.is_empty() {
            panic!("Unable to find latest log from empty list");
        }

        let mut found = &logs[0];
        logs.iter().for_each(|l| {
            if l.timestamp > found.timestamp {
                found = l;
            }
        });

        found.to_owned()
    }

    /// Helper for printing logs to the console.
    pub fn print_logs(&self, log_results: &[NewRelicLogItem]) {
        println!("Logs: \n-------\n");

        // clone the logs and make mutable so we can sort it
        let mut logs_copy = log_results.to_owned();

        // sort and print the logs
        logs_copy.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        logs_copy.iter().for_each(|row| {
            let t = row.timestamp.to_rfc3339();
            let mut log_message = row.message.clone();

            // clean up the logged string first
            log_message = log_message.replace('\\', "");

            // log to console
            println!(
                "{} - [{}] [{}] {}",
                t, row.logtype, row.logger_name, log_message
            )
        });
        println!("\n\n");
    }

    /// Get the timestamp in milliseconds of the NewRelicLogItem to use as a watermark
    pub fn to_watermark(&self, r: &NewRelicLogItem) -> String {
        // increment the timestamp by one ms
        let d = r.timestamp + Duration::milliseconds(1);
        format!("{}", d.timestamp_millis())
    }
}
