//! # Storage Module
//!
//! Handles saving and reading files to and from disk.
//!
//! ## Path
//!
//! storage.rs
//!
//! # Description
//!
//! Allows reading and writing data to files.

use chrono::{DateTime, Utc};
use tracing::{instrument, event, Level};
use std::cmp;
use std::path::{Path, PathBuf};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::env_config::{EnvConfig, LOG_DIRECTORY, LOG_FILE_EXTENSION, LOG_FILE_PREFIX};

/// New line character to check when reading files
const LF: u8 = b'\n';

/// Determines whether the given filename should be written to or the name
/// should be rolled over to another filename.
async fn should_rollover(filename: &str) -> bool {
    has_file(filename) && {
        let max_lines_per_file: usize = 1000;
        let num_lines = total_lines(filename).await.unwrap_or(0);
        num_lines >= max_lines_per_file
    }
}

fn get_log_dir() -> String {
    EnvConfig::global().get_val(LOG_DIRECTORY)
}

fn get_log_ext() -> String {
    EnvConfig::global().get_val(LOG_FILE_EXTENSION)
}

fn get_log_prefix() -> String {
    EnvConfig::global().get_val(LOG_FILE_PREFIX)
}

fn get_log_path(filename: &str) -> PathBuf {
    Path::new(&get_log_dir()).join(filename)
}

/// Creates the directory set via LOG_DIRECTORY configuration if it doesn't exist.
pub async fn ensure_log_directory() -> tokio::io::Result<()> {
    let dir_name = get_log_dir();
    let p = Path::new(&dir_name);
    if !p.exists() {
        tokio::fs::create_dir_all(p).await?;
    }
    Ok(())
}

/// Appends data to the log file with the given filename.
async fn append_to_file(filename: &str, data: &str) -> tokio::io::Result<()> {
    let filepath = get_log_path(filename);
    let mut file = OpenOptions::new().append(true).open(filepath).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

/// Writes data to a new log file with the given filename.
async fn write_to_new_file(filename: &str, data: &str) -> tokio::io::Result<()> {
    let filepath = get_log_path(filename);
    let mut file = File::create(filepath).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

/// Returns the list of log filenames.
/// Reads and returns the list of currently residing log files on the filesystem
/// under the folder configured via the `LOG_DIRECTORY` environment setting.
#[instrument(name="get_log_filenames")]
pub async fn get_log_filenames() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let log_location = get_log_dir();
    let p = Path::new(&log_location);
    if !p.exists() {
        event!(Level::ERROR, "Error: folder does not exist: {log_location}");
        return result;
    }

    let dir = std::fs::read_dir(p).expect("Unable to read directory");

    for entry in dir {
        let dir_entry = entry.unwrap();
        if dir_entry.metadata().unwrap().is_dir() {
            continue; // skip directories
        }
        let name = dir_entry.file_name();
        let entry_name = name.to_str().unwrap();
        result.push(entry_name.to_owned());
    }

    result
}

/// Determines whether the file at the given filename exists or not.
pub fn has_file(filename: &str) -> bool {
    get_log_path(filename).exists()
}

/// Writes a string to a file. Appends if file already exists.
pub async fn write_to_file(filename: &str, data: &str) -> tokio::io::Result<()> {
    ensure_log_directory()
        .await
        .expect("Unable to create log directory");

    if !has_file(filename) {
        write_to_new_file(filename, data).await
    } else {
        let data_with_newline = format!("\n{data}");
        append_to_file(filename, &data_with_newline).await
    }
}

#[instrument(name="delete_file")]
pub async fn delete_file(filename: &str) -> tokio::io::Result<()> {
    if !has_file(filename) {
        event!(Level::ERROR, "Unable to read file: {filename}");
        return Ok(());
    }

    tokio::fs::remove_file(get_log_path(filename)).await
}

/// Reads total lines of a file.
pub async fn total_lines(filename: &str) -> tokio::io::Result<usize> {
    let file = OpenOptions::new()
        .read(true)
        .open(get_log_path(filename))
        .await?;
    let mut f = BufReader::new(file);

    let mut count = 0;
    let z: usize = 0;
    let mut line: Vec<u8> = Vec::new();
    while match f.read_until(LF, &mut line).await {
        Ok(n) => n > z,
        Err(e) => return Err(e),
    } {
        count += 1;
    }
    Ok(count)
}

/// Generates a string to use as a filename. For not appending to log files and just creating mutliple
/// log files under the same date.
/// Uses the given timestamp and configured prefix and extension values to construct resulting filename.
/// If the resulting filename is already in use on the filesystem the name will used based on the `should_rollover`
/// policy and can be appended with an underscore and number if filename rollover is needed.
pub async fn get_filename(timestamp: DateTime<Utc>) -> String {
    let ext = get_log_ext();
    let prefix = get_log_prefix();
    let base_name = format!("{}_{}", prefix, timestamp.format("%Y-%m-%d"));

    let mut proposed_name = format!("{base_name}.{ext}");
    let mut incrementor = 0;

    // allow using existing filename based on rollover policy else increment with number
    loop {
        let rollover = should_rollover(&proposed_name).await;
        if !rollover {
            break;
        }
        incrementor += 1;
        proposed_name = format!("{base_name}_{incrementor}.{ext}");
    }
    proposed_name
}

/// Reads file with given filename from disk and returns the contents in lines.
/// Allows pagination through lines in the file via page and lines_per_page parameters.
/// Note: use `total_lines` fn for obtaining the total number of lines in a file.
pub async fn get_lines_by_page(filename: &str, page: u32, lines_per_page: u32) -> Vec<String> {
    let mut f = File::open(get_log_path(filename))
        .await
        .unwrap_or_else(|_| panic!("Couldn't open the file: {filename}"));

    let mut cursor = 0;
    let mut results: Vec<String> = Vec::new();

    // ensure params have sane values
    let normalized_page: u32 = cmp::max(1, page);
    let normalized_max_lines: u32 = cmp::max(1, lines_per_page);

    let mut reader = BufReader::new(&mut f);

    loop {
        cursor += 1;

        for _ in 0..normalized_max_lines {
            // read up to the end of the line
            let mut buffer = Vec::new();
            reader
                .read_until(LF, &mut buffer)
                .await
                .unwrap_or_else(|_| panic!("Problem reading lines in file {filename}!"));

            if !buffer.is_empty() && cursor == normalized_page {
                results.push(std::str::from_utf8(&buffer).unwrap_or("").to_owned());
            }
        }

        if cursor >= normalized_page {
            break;
        }
    }
    results
}
