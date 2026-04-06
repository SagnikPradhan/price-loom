use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use axum::routing::get;
use tracing::info;

use crate::bhav_nse::get_bhav_data_handler;
use crate::shared::config::AppConfig;
use crate::shared::storage::get_object_store;

mod bhav;
mod bhav_nse;
mod shared;

#[derive(Clone)]
pub struct AppState {
    object_store: Arc<dyn object_store::ObjectStore>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env().context("failed to load configuration")?;

    let object_store = get_object_store(config.object_store)?;
    let app = Router::new()
        .route("/bhav/nse/{date}", get(get_bhav_data_handler))
        .with_state(AppState { object_store });

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .with_context(|| format!("failed to bind to {}", address))?;

    info!("Listening on {:?}", address);
    axum::serve(listener, app).await.context("server crashed")?;

    Ok(())
}
