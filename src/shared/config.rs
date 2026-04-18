use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    pub object_store: String,
    pub database_url: String,
}

fn default_port() -> u16 {
    3000
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv()?;
        Ok(serde_env::from_env_with_prefix("PRICE_LOOM")?)
    }
}
