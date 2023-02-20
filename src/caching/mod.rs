

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
use mini_redis::{client::{self, Client}, Result};

pub const REDIS_URL: &'static str = "REDIS_URL";
pub const REDIS_CACH_KEY: &'static str = "last_seen_timestamp";

pub async fn get_redis_client() -> Result<Client> {
  let fallback_url = "127.0.0.1:6379".to_owned();
  let url = {
      match env::var(REDIS_URL) {
          Ok(t) => t,
          Err(_) => fallback_url,
      }
  };
  client::connect(url).await
}

async fn get_val(key_name: &str) -> Result<String> {
  let mut client = get_redis_client().await?;
  let result = client.get(key_name).await?;

  match result {
    Some(v) => Ok(format!("{:?}", v)),
    None => Ok("".to_owned())
  }
}

async fn set_val(key_name: &str, val: String) -> Result<()> {
  let mut client = get_redis_client().await?;
  client.set(key_name, val.into()).await?;
  Ok(())
}

pub async fn get_cached_val() -> Result<String> {
  Ok(get_val(REDIS_CACH_KEY).await?)
}

pub async fn set_cached_val(val: String) -> Result<()> {
  set_val(REDIS_CACH_KEY, val).await?;
  Ok(())
}
