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

use mini_redis::{
    client::{self, Client},
    Result,
};

use crate::env_config::{EnvConfig, REDIS_KEY_NAME, REDIS_URL};

pub async fn get_redis_client() -> Result<Client> {
    let redis_url = EnvConfig::global().get_val(REDIS_URL);
    client::connect(redis_url).await
}

async fn get_val(key_name: &str) -> Result<String> {
    let mut client = get_redis_client().await?;
    let result = client.get(key_name).await?;

    match result {
        Some(val_bytes) => {
            let val = std::str::from_utf8(val_bytes.as_ref()).unwrap_or("");
            Ok(val.to_owned())
        }
        None => Ok("".to_owned()),
    }
}

async fn set_val(key_name: &str, val: String) -> Result<()> {
    let mut client = get_redis_client().await?;
    client.set(key_name, val.into()).await?;
    Ok(())
}

pub async fn get_cached_val() -> Result<String> {
    let key_name = EnvConfig::global().get_val(REDIS_KEY_NAME);
    get_val(&key_name).await
}

pub async fn set_cached_val(val: String) -> Result<()> {
    let key_name = EnvConfig::global().get_val(REDIS_KEY_NAME);
    set_val(&key_name, val).await?;
    Ok(())
}
