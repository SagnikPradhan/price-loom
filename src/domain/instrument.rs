use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use sea_query::{IdenStatic, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_sqlx::SqlxBinder;
use sqlx::{PgTransaction, Row};
use uuid::Uuid;

use crate::types::{InstrumentSegment, InstrumentSource, InstrumentType};

#[enum_def]
struct Instrument {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub segment: InstrumentSegment,
    pub source: InstrumentSource,
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<String>,
    pub isin: String,
    pub ticker_symbol: String,
    pub security_series: String,
    pub instrument_name: Option<String>,
}

pub struct CreateInstrumentOptions {
    pub segment: InstrumentSegment,
    pub source: InstrumentSource,
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<String>,
    pub isin: String,
    pub ticker_symbol: String,
    pub security_series: String,
    pub instrument_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_instrument(
    trx: &mut PgTransaction<'_>,
    input: CreateInstrumentOptions,
) -> Result<Uuid> {
    let (sql, values) = Query::insert()
        .into_table(InstrumentIden::Table)
        .columns([
            InstrumentIden::Segment,
            InstrumentIden::Source,
            InstrumentIden::InstrumentType,
            InstrumentIden::InstrumentId,
            InstrumentIden::Isin,
            InstrumentIden::TickerSymbol,
            InstrumentIden::SecuritySeries,
            InstrumentIden::InstrumentName,
            InstrumentIden::CreatedAt,
            InstrumentIden::UpdatedAt,
        ])
        .values([
            input.segment.to_string().into(),
            input.source.to_string().into(),
            input.instrument_type.to_string().into(),
            input.instrument_id.into(),
            input.isin.into(),
            input.ticker_symbol.into(),
            input.security_series.into(),
            input.instrument_name.into(),
            input.created_at.into(),
            input.updated_at.into(),
        ])?
        .on_conflict(OnConflict::columns([
            InstrumentIden::Segment,
            InstrumentIden::InstrumentType,
            InstrumentIden::Isin,
        ]))
        .returning_col(InstrumentIden::Id)
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values).fetch_one(&mut **trx).await.unwrap();
    let id: Uuid = row.get(InstrumentIden::Id.as_str());

    Ok(id)
}
