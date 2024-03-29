//! # EnvConfig Module
//!
//! Handles access of configuration including environment variables.
//!
//! ## Path
//!
//! env_config.rs
//!
//! # Description
//!
//! This module allows retrieving configuration values read from environment
//! variables and defines the fallback values when environment variables are
//! not present.
//!
//! ## Notes
//!
//! Defines a `CONFIG` instance to be set only once on app start.

use once_cell::sync::OnceCell;
use std::{collections::HashMap, env};
use tracing::{event, instrument, Level};

/// The cell to init and hold the config instance (only writable once).
pub static CONFIG: OnceCell<EnvConfig> = OnceCell::new();

/// `api` env var name: the port the service will be served at.
pub const LS_SVC_PORT: &str = "LS_SVC_PORT";
/// `caching` env var name: the url redis is accessible at.
pub const REDIS_URL: &str = "REDIS_URL";
/// `caching` env var name: the hash_key to store the last seen timestamp under.
pub const REDIS_KEY_NAME: &str = "REDIS_KEY_NAME";
/// `cron_tasks` env var name: the schedule to poll for changes on the remote server.
pub const LS_POLL_SCHEDULE: &str = "LS_POLL_SCHEDULE";
/// `new_relic` env var name: the id of the new relic account the logs reside under.
pub const NRLS_ACCOUNT_ID: &str = "NRLS_ACCOUNT_ID";
/// `new_relic` env var name: the API key required to access the new relic query service endpoint.
pub const NRLS_API_KEY: &str = "NRLS_API_KEY";
/// `storage` env var name: the location of where logs are stored on the system.
pub const LOG_DIRECTORY: &str = "LOG_DIRECTORY";
/// `storage` env var name: filename prefix for saving log files.
pub const LOG_FILE_PREFIX: &str = "LOG_FILE_PREFIX";
/// `storage` env var name: the extension to use when saving log files.
pub const LOG_FILE_EXTENSION: &str = "LOG_FILE_EXTENSION";

/// Internal struct of `env_config` module for managing loading of environment
/// variables and mapping them if provided else falling back to defaults.
#[derive(Debug)]
pub struct EnvConfig<'a> {
    pub config: HashMap<&'a str, String>,
}

impl EnvConfig<'_> {
    /// Creates a new `EnvConfig` struct.
    pub fn new() -> EnvConfig<'static> {
        // initialize config with default values
        let mut new_instance = EnvConfig {
            config: HashMap::from([
                (LOG_DIRECTORY, "./".to_owned()),
                (LOG_FILE_PREFIX, "app".to_owned()),
                (LOG_FILE_EXTENSION, "log".to_owned()),
                (LS_POLL_SCHEDULE, "0 1/5 * * * *".to_owned()),
                (LS_SVC_PORT, "3333".to_owned()),
                (NRLS_ACCOUNT_ID, "".to_owned()),
                (NRLS_API_KEY, "".to_owned()),
                (REDIS_URL, "127.0.0.1:6379".to_owned()),
                (REDIS_KEY_NAME, "last_seen_timestamp".to_owned()),
            ]),
        };
        new_instance.read_env_config();
        new_instance
    }

    /// Reads environment variables and updates config values.
    #[instrument(name = "read_env_config")]
    pub fn read_env_config(&mut self) {
        let keys = self.config.keys().copied().collect::<Vec<&str>>();
        for k in keys {
            match env::var(k) {
                Ok(val) => {
                    event!(Level::INFO, "Using config value {k} (environment) = {val}");
                    self.config.insert(k, val);
                }
                Err(_) => {
                    let v = self.config.get(k).unwrap();
                    event!(Level::WARN, "Using config value {k} (default) = {v}");
                }
            };
        }
    }

    /// Retrieves a config value.
    pub fn get_val(&self, env_var: &str) -> String {
        if !self.config.contains_key(env_var) {
            panic!(
                "Unknown environment variable {env_var}. {}",
                "Try adding it to the validation whitelist."
            );
        }
        match self.config.get(env_var) {
            Some(val) => val.to_owned(),
            None => panic!("Config value for '{env_var}' not found!"),
        }
    }

    /// Get the current instance of the EnvConfig struct.
    pub fn global() -> &'static EnvConfig<'static> {
        CONFIG.get().expect("Unable to resolve EnvConfig instance!")
    }
}
