//! # Storage Module
//!
//! Handles saving and reading files to disk.
//!
//! ## Path
//!
//! storage/mod.rs
//!
//! # Description
//!
//! Allows storing a value via Redis using the REDIS_CACH_KEY.

use chrono::NaiveDate;

use crate::env_config::{self, LOG_FILE_EXTENSION, LOG_FILE_PREFIX};

/// Returns the list of log filenames.
/// Reads and returns the list of currently residing log files on the filesystem
/// under the folder configured via the `LOG_DIRECTORY` environment setting.
pub fn get_log_filenames() -> Vec<String> {
    let result: Vec<String> = [].to_vec();
    // TODO: implement this
    return result;
}

/// Reads file with given filename from disk and returns the contents.
pub fn get_log_contents(filename: &str) -> String {
    // TODO: implement this
    return "".to_owned();
}

/// Determines whether the given log file should be appended to or not.
pub fn can_append_file(filename: &str) -> bool {
    // TODO: implement this
    return true;
}

/// Appends data to the log file with the given filename.
pub fn append_to_file(filename: &str, data: &str) -> bool {
    // TODO: implement this
    return true;
}

/// Writes data to a new log file with the given filename.
pub fn to_new_file(filename: &str, data: &str) -> bool {
    // TODO: implement this
    return true;
}

/// Generates a string to use as a filename.
/// Uses the given timestamp and configured prefix and extension values to construct resulting filename.
/// If the resulting filename is already in use on the filesystem the name will be appended with a number
/// to avoid collisions.
pub fn to_filename(timestamp: NaiveDate) -> String {
    // TODO: centralize environment variable names to one location
    let ext = env_config::get_var_else(LOG_FILE_EXTENSION, "log");
    let prefix = env_config::get_var_else(LOG_FILE_PREFIX, "app");

    let proposed_name = format!("{}_{}_log.{}", prefix, timestamp.format("%Y-%m-%d"), ext);

    // TODO: ensure filename doesn't already exist and increment with a number until we have
    // a unique filename to use
    proposed_name
}
