use anyhow::{Context, Result};
use axum::Router;
use axum::routing::get;
use tracing::info;

use crate::services::bhav::get_bhav_data_handler;
use crate::shared::config::AppConfig;
use crate::shared::database::get_db_connection;
use crate::shared::storage::get_object_store;
use crate::types::AppState;

mod domain;
mod services;
mod shared;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env().context("failed to load configuration")?;

    let db = get_db_connection(&config.connection_uri).await?;
    let object_store = get_object_store(config.object_store)?;
    let app = Router::new()
        .route("/bhav/nse/{date}", get(get_bhav_data_handler))
        .with_state(AppState { object_store, db });

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .with_context(|| format!("failed to bind to {}", address))?;

    info!("Listening on {:?}", address);
    axum::serve(listener, app).await.context("server crashed")?;

    Ok(())
}
