use std::{collections::HashSet, env};

// storage env vars
pub const LOG_DIRECTORY: &'static str = "LOG_DIRECTORY";
pub const LOG_FILE_PREFIX: &'static str = "LOG_FILE_PREFIX";
pub const LOG_FILE_EXTENSION: &'static str = "LOG_FILE_EXTENSION";
// new relic env vars
pub const NRLS_ACCOUNT_ID: &'static str = "NRLS_ACCOUNT_ID";
pub const NRLS_API_KEY: &'static str = "NRLS_API_KEY";
// cache env vars
pub const REDIS_URL: &'static str = "REDIS_URL";
pub const REDIS_KEY_NAME: &'static str = "REDIS_KEY_NAME";
// server env vars
pub const LS_SVC_PORT: &'static str = "LS_SVC_PORT";

/// Checks that the given env_var value is in the whitelist of known
/// environment variable names.
fn validate_var_name(env_var: &str) {
    let whitelist: HashSet<&'static str> = HashSet::from([
        LOG_DIRECTORY,
        LOG_FILE_PREFIX,
        LOG_FILE_EXTENSION,
        NRLS_ACCOUNT_ID,
        NRLS_API_KEY,
        REDIS_URL,
        REDIS_KEY_NAME,
        LS_SVC_PORT,
    ]);
    if !whitelist.contains(env_var) {
        panic!(
            "Unknown environment variable {env_var}. {}",
            "Try adding it to the validation whitelist."
        );
    }
}

/// Attempts to read the environment variable with the given env_var name.
/// Returns the fallback_value if environment variables doesn't exist.
pub fn get_var_else(env_var: &str, fallback_value: &str) -> String {
    validate_var_name(env_var);
    env::var(env_var).unwrap_or_else(|_| fallback_value.to_string())
}
