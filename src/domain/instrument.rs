use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use sqlx::PgTransaction;
use uuid::Uuid;

use crate::types::{InstrumentSegment, InstrumentSource, InstrumentType};

pub struct CreateInstrument {
    pub segment: InstrumentSegment,
    pub source: InstrumentSource,
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<String>,
    pub isin: Option<String>,
    pub ticker_symbol: String,
    pub security_series: String,
    pub instrument_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_instrument(
    trx: &mut PgTransaction<'_>,
    input: CreateInstrument,
) -> Result<Uuid> {
    let row = sqlx::query_scalar!(
        "
            INSERT INTO instrument (
                segment,
                source,
                instrument_type,
                instrument_id,
                isin,
                ticker_symbol,
                security_series,
                instrument_name,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (source, instrument_type, isin)
            DO UPDATE SET
                segment = EXCLUDED.segment,
                instrument_id = EXCLUDED.instrument_id,
                ticker_symbol = EXCLUDED.ticker_symbol,
                security_series = EXCLUDED.security_series,
                instrument_name = EXCLUDED.instrument_name
            RETURNING id
        ",
        input.segment as _,
        input.source as _,
        input.instrument_type as _,
        input.instrument_id,
        input.isin,
        input.ticker_symbol,
        input.security_series,
        input.instrument_name,
        input.created_at,
        input.updated_at,
    )
    .fetch_one(&mut **trx)
    .await
    .unwrap();

    Ok(row)
}
