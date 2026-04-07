use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "PRICE_LOOM_PORT", default = "default_port")]
    pub port: u16,
    #[serde(rename = "PRICE_LOOM_OBJECT_STORE")]
    pub object_store: String,
    #[serde(rename = "PRICE_LOOM_DATABASE_URL")]
    pub connection_uri: String,
}

fn default_port() -> u16 {
    3000
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(serde_env::from_env()?)
    }
}
