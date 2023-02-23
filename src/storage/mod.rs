//! # Caching Utilities Module
//!
//! Handles connection to redis and chaching values.
//!
//! ## Path
//!
//! caching/mod.rs
//!
//! # Description
//!
//! Allows caching a value via Redis using the REDIS_CACH_KEY.

use std::env;

use chrono::NaiveDate;

pub const LOG_DIRECTORY: &'static str = "LOG_DIRECTORY";
pub const LOG_FILE_PREFIX: &'static str = "LOG_FILE_PREFIX";
pub const LOG_FILE_EXTENSION: &'static str = "LOG_FILE_EXTENSION";

pub const FALLBACK_LOG_PREFIX: &'static str = "scraped";
pub const FALLBACK_LOG_EXT: &'static str = "log";
pub const FALLBACK_LOG_DIRECTORY: &'static str = ".";

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
    let ext = {
        match env::var(LOG_FILE_EXTENSION) {
            Ok(t) => t,
            Err(_) => FALLBACK_LOG_EXT.to_owned(),
        }
    };
    let prefix = {
        match env::var(LOG_FILE_PREFIX) {
            Ok(t) => t,
            Err(_) => FALLBACK_LOG_PREFIX.to_owned(),
        }
    };

    let proposedFilename = format!("{}_{}_log.{}", prefix, timestamp.format("%Y-%m-%d"), ext);

    // TODO: ensure filename doesn't already exist and increment with a number until we have
    // a unique filename to use
    proposedFilename
}
