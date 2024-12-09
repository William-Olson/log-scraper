//! # Caching Utilities Module
//!
//! Handles connection to redis and chaching values.
//!
//! ## Path
//!
//! caching.rs
//!
//! # Description
//!
//! Allows caching a value via Redis using the REDIS_CACH_KEY.

use redis::Commands;
use tracing::{trace, instrument};

use crate::env_config::{EnvConfig, REDIS_KEY_NAME, REDIS_URL};

#[instrument(name = "establish_redis_connection")]
pub async fn get_redis_client() -> Result<redis::Connection, String> {
    let redis_config_url = EnvConfig::global().get_val(REDIS_URL);
    let redis_url_with_prototype = format!("redis://{redis_config_url}");
    trace!("Connecting to redis at {}", redis_url_with_prototype);
    let client = match redis::Client::open(redis_url_with_prototype) {
        Ok(c) => c,
        Err(e) => match e.detail() {
            Some(d) => return Err(d.to_owned()),
            None => return Err(format!("Error Obtaining Redis Client {:?}", e).to_owned()),
        }
    };
    let connection = match client.get_connection() {
        Ok(c) => c,
        Err(e) => match e.detail() {
            Some(d) => return Err(d.to_owned()),
            None => return Err(format!("Error Obtaining Redis Connection {:?}", e).to_owned()),
        }
    };
    Ok(connection)
}

async fn get_val(key_name: &str) -> Result<String, String> {
    let mut connection: redis::Connection = match get_redis_client().await {
        Ok(c) => c,
        Err(s) => return Err(s),
    };

    let result: redis::RedisResult<String> = connection.get::<&str, String>(key_name);

    match result {
        Ok(string_value) => Ok(string_value.to_owned()),
        Err(_) => Ok("".to_owned())
    }
}

async fn set_val(key_name: &str, val: String) -> Result<(), String> {
    let mut connection: redis::Connection = match get_redis_client().await {
        Ok(c) => c,
        Err(s) => return Err(s),
    };
    match connection.set::<&str, String, ()>(key_name, val) {
        Ok(_) => Ok(()),
        Err(s) => match s.detail() {
            Some(d) => Err(d.to_owned()),
            None => Err("Unknown error attempting to set value".to_owned()),
        }
    }
}

pub async fn get_cached_val() -> Result<String, String> {
    let key_name = EnvConfig::global().get_val(REDIS_KEY_NAME);
    get_val(&key_name).await
}

pub async fn set_cached_val(val: String) -> Result<(), String> {
    let key_name = EnvConfig::global().get_val(REDIS_KEY_NAME);
    set_val(&key_name, val).await?;
    Ok(())
}
