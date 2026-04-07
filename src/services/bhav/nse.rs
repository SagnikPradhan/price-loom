use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use bytes::Bytes;
use chrono::NaiveDate;
use object_store::ObjectStore;
use reqwest::StatusCode;
use serde_json::json;
use tracing::{debug, info, instrument, warn};

use crate::shared::storage::{SaveSourceFileOptions, Source, read_file, save_file};
use crate::types::{Bhav, BhavRecord};

#[instrument(skip_all)]
pub async fn get_bhav_data(
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
    if let Some(data) = read_file(store, &date, &Source::NSE).await? {
        return Ok(Some(data));
    }

    let fetched = fetch_bhav_file(date).await?;
    let Some(data) = fetched else {
        return Ok(None);
    };

    let bytes = Bytes::from(data);
    save_file(store, SaveSourceFileOptions { date, kind: Source::NSE, data: bytes.clone() })
        .await?;

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
