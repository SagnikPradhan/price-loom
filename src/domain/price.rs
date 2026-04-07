use anyhow::{Ok, Result};
use chrono::{DateTime, NaiveDate, Utc};
use sea_query::{IdenStatic, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_sqlx::SqlxBinder;
use sqlx::{PgTransaction, Row};
use uuid::Uuid;

#[enum_def]
struct Price {
    pub id: Uuid,
    pub instrument_id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub trade_date: NaiveDate,
    pub business_date: Option<NaiveDate>,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub previous_close_price: f64,
    pub total_traded_volume: u64,
    pub total_traded_value: f64,
    pub total_number_of_trades: u64,
    pub session_id: Option<String>,
    pub market_lot_size: Option<u64>,
    pub remarks: Option<String>,
}

pub struct CreatePriceOptions {
    pub instrument_id: Uuid,
    pub file_id: Uuid,
    pub trade_date: NaiveDate,
    pub business_date: Option<NaiveDate>,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub previous_close_price: f64,
    pub total_traded_volume: u64,
    pub total_traded_value: f64,
    pub total_number_of_trades: u64,
    pub session_id: Option<String>,
    pub market_lot_size: Option<u64>,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_price(trx: &mut PgTransaction<'_>, input: CreatePriceOptions) -> Result<Uuid> {
    let (sql, values) = Query::insert()
        .into_table(PriceIden::Table)
        .columns([
            PriceIden::InstrumentId,
            PriceIden::FileId,
            PriceIden::TradeDate,
            PriceIden::BusinessDate,
            PriceIden::OpenPrice,
            PriceIden::HighPrice,
            PriceIden::LowPrice,
            PriceIden::ClosePrice,
            PriceIden::LastPrice,
            PriceIden::PreviousClosePrice,
            PriceIden::TotalTradedVolume,
            PriceIden::TotalTradedValue,
            PriceIden::TotalNumberOfTrades,
            PriceIden::SessionId,
            PriceIden::MarketLotSize,
            PriceIden::Remarks,
            PriceIden::CreatedAt,
            PriceIden::UpdatedAt,
        ])
        .values([
            input.instrument_id.into(),
            input.file_id.into(),
            input.trade_date.into(),
            input.business_date.into(),
            input.open_price.into(),
            input.high_price.into(),
            input.low_price.into(),
            input.close_price.into(),
            input.last_price.into(),
            input.previous_close_price.into(),
            input.total_traded_volume.into(),
            input.total_traded_value.into(),
            input.total_number_of_trades.into(),
            input.session_id.into(),
            input.market_lot_size.into(),
            input.remarks.into(),
            input.created_at.into(),
            input.updated_at.into(),
        ])?
        .on_conflict(OnConflict::columns([PriceIden::InstrumentId, PriceIden::TradeDate]))
        .returning_col(PriceIden::Id)
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values).fetch_one(&mut **trx).await.unwrap();
    let id: Uuid = row.get(PriceIden::Id.as_str());

    Ok(id)
}
