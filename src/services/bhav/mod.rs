use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Path, State};
use chrono::NaiveDate;

use crate::services::bhav::nse::get_bhav_data;
use crate::shared::error::AppError;
use crate::types::{AppState, Bhav};

pub mod nse;

pub async fn get_bhav_data_handler(
    Path(date_str): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Bhav>, AppError> {
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").context("Found invalid date!")?;

    get_bhav_data(&state.object_store, date)
        .await?
        .map(|(_, data)| Json(data))
        .context("Found no bhavcopy!")
        .map_err(AppError::from)
}
