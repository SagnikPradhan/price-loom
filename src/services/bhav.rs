use anyhow::{Context, Ok, Result};
use axum::Json;
use axum::extract::{Path, State};
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::adapters::bhav::{fetch_bhav_file, parse_bhav_csv_records};
use crate::adapters::store::{SaveSourceFileOptions, read_file, save_file};
use crate::domain::instrument::{CreateInstrument, create_instrument};
use crate::domain::price::{CreatePrice, create_price};
use crate::domain::source_file::{CreateSourceFile, create_source_file};
use crate::shared::error::AppError;
use crate::types::{AppState, Bhav, BhavRecord, InstrumentSource};

#[axum::debug_handler]
pub async fn get_bhav_data_handler(
    Path((source, date_str)): Path<(InstrumentSource, String)>,
    State(state): State<AppState>,
) -> Result<Json<Bhav>, AppError> {
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").context("Found invalid date!")?;

    get_bhav_data(&state, &source, &date)
        .await?
        .map(Json)
        .context("Found no bhavcopy!")
        .map_err(AppError::from)
}

#[instrument(skip_all)]
pub async fn get_bhav_data(
    state: &AppState,
    source: &InstrumentSource,
    date: &NaiveDate,
) -> Result<Option<Bhav>> {
    if let Some((path, data)) = read_file(&state.object_store, date, source).await? {
        return Ok(Some(Bhav {
            key: path.to_string(),
            date: date.clone(),
            source: source.clone(),
            prices: parse_bhav_csv_records(data)?,
        }));
    }

    let Some(data) = fetch_bhav_file(source, date).await? else {
        return Ok(None);
    };

    let file = SaveSourceFileOptions { date: date.clone(), source: *source, data: data.clone() };
    let key = save_file(&state.object_store, file).await?;

    let bhav = Bhav {
        key: key.to_string(),
        date: date.clone(),
        source: source.clone(),
        prices: parse_bhav_csv_records(data)?,
    };

    persist_bhav_data(&state.db, &bhav).await?;
    Ok(Some(bhav))
}

#[instrument(skip_all)]
pub async fn persist_bhav_data(db: &PgPool, bhav: &Bhav) -> Result<()> {
    let mut trx = db.begin().await?;

    let file = create_source_file(&mut trx, bhav.into_file()).await?;
    for price in bhav.prices.iter().clone() {
        let instrument = create_instrument(&mut trx, price.into_instrument()?).await?;
        create_price(&mut trx, price.into_create_price(instrument, file)).await?;
    }

    trx.commit().await?;
    Ok(())
}

impl Bhav {
    fn into_file(&self) -> CreateSourceFile {
        CreateSourceFile {
            source: self.source,
            date: self.date,
            key: self.key.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl BhavRecord {
    fn into_create_price(&self, instrument: Uuid, file: Uuid) -> CreatePrice {
        CreatePrice {
            instrument_id: instrument,
            file_id: file,
            trade_date: self.trade_date,
            business_date: self.business_date,
            open_price: self.open_price,
            high_price: self.high_price,
            low_price: self.low_price,
            close_price: self.close_price,
            last_price: self.last_price,
            previous_close_price: self.previous_close_price,
            total_traded_volume: self.total_traded_volume,
            total_traded_value: self.total_traded_value,
            total_number_of_trades: self.total_number_of_trades,
            session_id: self.session_id.clone(),
            market_lot_size: self.market_lot_size,
            remarks: self.remarks.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn into_instrument(&self) -> Result<CreateInstrument> {
        Ok(CreateInstrument {
            segment: self.segment,
            source: self.source,
            instrument_type: self.instrument_type,
            instrument_id: self.instrument_id.clone(),
            isin: self.isin.clone(),
            ticker_symbol: self.ticker_symbol.clone(),
            security_series: self.security_series.clone(),
            instrument_name: self.instrument_name.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
