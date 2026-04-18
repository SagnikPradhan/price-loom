use std::collections::BTreeSet;
use std::sync::Arc;

use chrono::NaiveDate;
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use sqlx::{Pool, Postgres};
use strum::Display;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub object_store: Arc<dyn object_store::ObjectStore>,
}

#[derive(Debug, Serialize)]
pub struct Bhav {
    pub key: String,
    pub date: NaiveDate,
    pub source: InstrumentSource,
    pub prices: BTreeSet<BhavRecord>,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BhavRecord {
    pub source: InstrumentSource,
    pub segment: InstrumentSegment,
    pub instrument_type: InstrumentType,
    pub trade_date: NaiveDate,
    pub ticker_symbol: String,
    pub isin: Option<String>,

    pub business_date: Option<NaiveDate>,
    pub instrument_id: Option<String>,
    pub security_series: String,
    pub expiry_date: Option<String>,
    pub actual_expiry_date: Option<String>,
    pub strike_price: Option<NotNan<f64>>,
    pub option_type: Option<String>,
    pub instrument_name: Option<String>,
    pub open_price: NotNan<f64>,
    pub high_price: NotNan<f64>,
    pub low_price: NotNan<f64>,
    pub close_price: NotNan<f64>,
    pub last_price: NotNan<f64>,
    pub previous_close_price: NotNan<f64>,
    pub underlying_price: Option<NotNan<f64>>,
    pub settlement_price: Option<NotNan<f64>>,
    pub open_interest: Option<i64>,
    pub change_in_open_interest: Option<i64>,
    pub total_traded_volume: i64,
    pub total_traded_value: NotNan<f64>,
    pub total_number_of_trades: Option<i64>,
    pub session_id: Option<String>,
    pub market_lot_size: Option<i64>,
    pub remarks: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
#[sqlx(type_name = "instrument_type")]
pub enum InstrumentType {
    #[sqlx(rename = "CUR")]
    #[serde(rename = "CUR")]
    Currency,
    #[sqlx(rename = "CDF")]
    #[serde(rename = "CDF")]
    CurrencyFutures,
    #[sqlx(rename = "CDO")]
    #[serde(rename = "CDO")]
    CurrencyOptions,
    #[sqlx(rename = "IRF")]
    #[serde(rename = "IRF")]
    InterestRateFuturesMiborGsec,
    #[sqlx(rename = "IRT")]
    #[serde(rename = "IRT")]
    InterestRateFuturesTbill,
    #[sqlx(rename = "IRO")]
    #[serde(rename = "IRO")]
    InterestRateOptions,
    #[sqlx(rename = "STK")]
    #[serde(rename = "STK")]
    Stock,
    #[sqlx(rename = "COM")]
    #[serde(rename = "COM")]
    Commodity,
    #[sqlx(rename = "COF")]
    #[serde(rename = "COF")]
    CommodityFutures,
    #[sqlx(rename = "COO")]
    #[serde(rename = "COO")]
    CommodityOptions,
    #[sqlx(rename = "FUO")]
    #[serde(rename = "FUO")]
    OptionsOnFutures,
    #[sqlx(rename = "STF")]
    #[serde(rename = "STF")]
    StockFutures,
    #[sqlx(rename = "STO")]
    #[serde(rename = "STO")]
    StockOptions,
    #[sqlx(rename = "IDF")]
    #[serde(rename = "IDF")]
    IndexFutures,
    #[sqlx(rename = "IDO")]
    #[serde(rename = "IDO")]
    IndexOptions,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type)]
#[sqlx(type_name = "instrument_segment")]
pub enum InstrumentSegment {
    #[sqlx(rename = "CM")]
    #[serde(rename = "CM")]
    CaptialMarkets,
    #[sqlx(rename = "FO")]
    #[serde(rename = "FO")]
    FuturesAndOptions,
    #[sqlx(rename = "CD")]
    #[serde(rename = "CD")]
    CurrencyDerivatives,
    #[sqlx(rename = "COM")]
    #[serde(rename = "COM")]
    CommoditiesDerivatives,
}

#[derive(
    Clone, Copy, Debug, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Type,
)]
#[sqlx(type_name = "source")]
#[sqlx(rename_all = "lowercase")]
pub enum InstrumentSource {
    #[serde(rename = "NSE")]
    NSE,
}

pub struct Instrument {
    pub segment: InstrumentSegment,
    pub source: InstrumentSource,
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<String>,
    pub isin: String,
    pub ticker_symbol: String,
    pub security_series: String,
    pub instrument_name: Option<String>,
}

pub struct Price {
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
    pub total_number_of_trades: i64,
    pub session_id: Option<String>,
    pub market_lot_size: Option<i64>,
    pub remarks: Option<String>,
}
