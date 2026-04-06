use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::Json;
use axum::extract::{Path, State};
use bytes::Bytes;
use chrono::NaiveDate;
use object_store::ObjectStore;
use reqwest::StatusCode;
use serde_json::json;
use tracing::{debug, info, instrument, warn};

use crate::AppState;
use crate::bhav::{Bhav, BhavRecord};
use crate::shared::error::AppError;
use crate::shared::storage::{AppFile, AppFileKind, read_file, save_file};

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

#[instrument(skip_all)]
async fn get_bhav_data(
    store: &Arc<dyn ObjectStore>,
    date: NaiveDate,
) -> Result<Option<(Bytes, Bhav)>> {
    info!("Fetching bhavcopy {:?}", date);

    let Some(csv) = get_or_fetch_nse_bhav(store, date).await? else {
        info!("No bhavcopy available for {:?}", date);
        return Ok(None);
    };

    let records = parse_bhav_csv(&csv)?;
    info!("Parsed {:?} records for {:?}", records.len(), date);
    Ok(Some((csv, Bhav { prices: records })))
}

#[instrument(skip_all)]
pub async fn get_or_fetch_nse_bhav(
    store: &Arc<dyn ObjectStore>,
    date: NaiveDate,
) -> Result<Option<Bytes>> {
    if let Some(data) = read_file(store, &date, &AppFileKind::NSE).await? {
        return Ok(Some(data));
    }

    let fetched = fetch_bhav_file(date).await?;
    let Some(data) = fetched else {
        return Ok(None);
    };

    let bytes = Bytes::from(data);
    save_file(store, AppFile { date, kind: AppFileKind::NSE, data: bytes.clone() }).await?;

    Ok(Some(bytes))
}

fn get_bhav_file_url(date: NaiveDate) -> String {
    let cutoff = NaiveDate::from_ymd_opt(2024, 6, 1).expect("Valid fixed NSE cutoff date");

    let file = if date > cutoff {
        json!([{
            "name": "CM-UDiFF Common Bhavcopy Final (zip)",
            "type": "daily-reports",
            "category": "capital-market",
            "section": "equities"
        }])
    } else {
        json!([{
            "name": "CM - Bhavcopy(csv)",
            "type": "archives",
            "category": "capital-market",
            "section": "equities"
        }])
    };

    format!(
        "https://www.nseindia.com/api/reports?archives={}&date={}&type=equities&mode=single",
        file.to_string(),
        date.format("%d-%b-%Y")
    )
}

#[instrument(skip_all)]
async fn fetch_bhav_file(date: NaiveDate) -> Result<Option<Vec<u8>>> {
    let url = get_bhav_file_url(date);
    debug!("Requesting bhav file - {:?}", url);

    let response = reqwest::get(&url)
        .await
        .with_context(|| format!("Failed to fetch bhavcopy for {}", date))?;

    if response.status() == StatusCode::NOT_FOUND {
        warn!("Could not resolve bhav file - {:?}", date);
        return Ok(None);
    }

    let bytes = response
        .error_for_status()
        .with_context(|| format!("NSE returned error for {}", date))?
        .bytes()
        .await
        .with_context(|| format!("Failed reading body for {}", date))?
        .to_vec();

    debug!("Downloaded bhav file {:?}", date);
    if bytes.starts_with(b"PK") { Ok(Some(unzip_file(&bytes)?)) } else { Ok(Some(bytes.to_vec())) }
}

fn unzip_file(zip_bytes: &[u8]) -> Result<Vec<u8>> {
    use std::io::{Cursor, Read};

    use zip::ZipArchive;

    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)?;

    if archive.len() == 0 {
        anyhow::bail!("Empty zip archive");
    }

    let mut file = archive.by_index(0)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn parse_bhav_csv(bytes: &[u8]) -> Result<Vec<BhavRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(bytes);

    let headers = rdr.headers()?.iter().take_while(|h| !h.is_empty()).collect();
    rdr.set_headers(headers);
    Ok(rdr.deserialize().flatten().collect())
}
