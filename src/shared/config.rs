use std::env;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct AppConfig {
    pub port: u16,
    pub object_store: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv()?;

        Ok(Self {
            port: get_env_parse("PRICE_LOOM_PORT").unwrap_or(3000),
            object_store: get_env("PRICE_LOOM_OBJECT_STORE")?,
        })
    }
}

fn get_env(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("Missing required env var `{key}`"))
}

fn get_env_parse<T>(key: &str) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let value = get_env(key)?;
    value.parse::<T>().with_context(|| format!("Invalid value for `{key}`: `{value}`"))
}
