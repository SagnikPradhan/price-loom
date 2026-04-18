use anyhow::{Ok, Result};
use chrono::{DateTime, NaiveDate, Utc};
use ordered_float::NotNan;
use sqlx::PgTransaction;
use uuid::Uuid;

pub struct CreatePrice {
    pub instrument_id: Uuid,
    pub file_id: Uuid,
    pub trade_date: NaiveDate,
    pub business_date: Option<NaiveDate>,
    pub open_price: NotNan<f64>,
    pub high_price: NotNan<f64>,
    pub low_price: NotNan<f64>,
    pub close_price: NotNan<f64>,
    pub last_price: NotNan<f64>,
    pub previous_close_price: NotNan<f64>,
    pub total_traded_volume: i64,
    pub total_traded_value: NotNan<f64>,
    pub total_number_of_trades: Option<i64>,
    pub session_id: Option<String>,
    pub market_lot_size: Option<i64>,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_price(trx: &mut PgTransaction<'_>, input: CreatePrice) -> Result<Uuid> {
    let row = sqlx::query!(
        "
            INSERT INTO price (
                instrument_id,
                file_id,
                trade_date,
                business_date,
                open_price,
                high_price,
                low_price,
                close_price,
                last_price,
                previous_close_price,
                total_traded_volume,
                total_traded_value,
                total_number_of_trades,
                session_id,
                market_lot_size,
                remarks,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9,
                $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            ON CONFLICT (instrument_id, trade_date) DO NOTHING
            RETURNING id
        ",
        input.instrument_id,
        input.file_id,
        input.trade_date,
        input.business_date,
        input.open_price.into_inner(),
        input.high_price.into_inner(),
        input.low_price.into_inner(),
        input.close_price.into_inner(),
        input.last_price.into_inner(),
        input.previous_close_price.into_inner(),
        input.total_traded_volume,
        input.total_traded_value.into_inner(),
        input.total_number_of_trades,
        input.session_id,
        input.market_lot_size,
        input.remarks,
        input.created_at,
        input.updated_at
    )
    .fetch_one(&mut **trx)
    .await
    .unwrap();

    Ok(row.id)
}
