use std::collections::BTreeSet;
use std::io::{Cursor, Read};

use anyhow::{Context, Ok, Result};
use bytes::{Buf, Bytes};
use chrono::NaiveDate;
use ordered_float::NotNan;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, warn};
use zip::ZipArchive;

use crate::types::{BhavRecord, InstrumentSegment, InstrumentSource, InstrumentType};

#[derive(Debug, Serialize, Deserialize)]
pub struct BhavRawRecord {
    #[serde(rename = "TradDt", alias = "TIMESTAMP")]
    pub trade_date: String,
    #[serde(rename = "BizDt")]
    pub business_date: Option<String>,
    #[serde(rename = "Sgmt")]
    pub segment: Option<InstrumentSegment>,
    #[serde(rename = "Src")]
    pub source: Option<InstrumentSource>,
    #[serde(rename = "FinInstrmTp", alias = "INSTRUMENT")]
    pub instrument_type: Option<InstrumentType>,
    #[serde(rename = "FinInstrmId")]
    pub instrument_id: Option<String>,
    #[serde(rename = "ISIN")]
    pub isin: Option<String>,
    #[serde(rename = "TckrSymb", alias = "SYMBOL")]
    pub ticker_symbol: String,
    #[serde(rename = "SctySrs", alias = "SERIES")]
    pub security_series: String,
    #[serde(rename = "XpryDt", alias = "EXPIRY_DT")]
    pub expiry_date: Option<String>,
    #[serde(rename = "FininstrmActlXpryDt")]
    pub actual_expiry_date: Option<String>,
    #[serde(rename = "StrkPric", alias = "STRIKE_PR")]
    pub strike_price: Option<f64>,
    #[serde(rename = "OptnTp", alias = "OPTION_TYP")]
    pub option_type: Option<String>,
    #[serde(rename = "FinInstrmNm")]
    pub instrument_name: Option<String>,
    #[serde(rename = "OpnPric", alias = "OPEN")]
    pub open_price: f64,
    #[serde(rename = "HghPric", alias = "HIGH")]
    pub high_price: f64,
    #[serde(rename = "LwPric", alias = "LOW")]
    pub low_price: f64,
    #[serde(rename = "ClsPric", alias = "CLOSE")]
    pub close_price: f64,
    #[serde(rename = "LastPric", alias = "LAST")]
    pub last_price: f64,
    #[serde(rename = "PrvsClsgPric", alias = "PREVCLOSE")]
    pub previous_close_price: f64,
    #[serde(rename = "UndrlygPric")]
    pub underlying_price: Option<f64>,
    #[serde(rename = "SttlmPric", alias = "SETTLE_PR")]
    pub settlement_price: Option<f64>,
    #[serde(rename = "OpnIntrst", alias = "OPEN_INT")]
    pub open_interest: Option<i64>,
    #[serde(rename = "ChngInOpnIntrst", alias = "CHG_IN_OI")]
    pub change_in_open_interest: Option<i64>,
    #[serde(rename = "TtlTradgVol", alias = "TOTTRDQTY", alias = "CONTRACTS")]
    pub total_traded_volume: i64,
    #[serde(rename = "TtlTrfVal", alias = "TOTTRDVAL", alias = "VAL_INLAKH")]
    pub total_traded_value: f64,
    #[serde(rename = "TtlNbOfTxsExctd", alias = "TOTALTRADES")]
    pub total_number_of_trades: Option<i64>,
    #[serde(rename = "SsnId")]
    pub session_id: Option<String>,
    #[serde(rename = "NewBrdLotQty")]
    pub market_lot_size: Option<i64>,
    #[serde(rename = "Rmks")]
    pub remarks: Option<String>,
}

impl BhavRecord {
    pub fn from_raw(r: BhavRawRecord, source: InstrumentSource) -> anyhow::Result<Self> {
        Ok(Self {
            source,
            segment: r.segment.unwrap_or_else(|| InstrumentSegment::CaptialMarkets),
            instrument_type: r.instrument_type.unwrap_or_else(|| InstrumentType::Stock),
            isin: r.isin,
            trade_date: NaiveDate::parse_from_str(&r.trade_date, "%d-%b-%Y")?,
            business_date: r
                .business_date
                .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
                .transpose()?,
            instrument_id: r.instrument_id,
            ticker_symbol: r.ticker_symbol,
            security_series: r.security_series,
            expiry_date: r.expiry_date,
            actual_expiry_date: r.actual_expiry_date,
            strike_price: r.strike_price.map(NotNan::new).transpose()?,
            option_type: r.option_type,
            instrument_name: r.instrument_name,
            open_price: NotNan::new(r.open_price)?,
            high_price: NotNan::new(r.high_price)?,
            low_price: NotNan::new(r.low_price)?,
            close_price: NotNan::new(r.close_price)?,
            last_price: NotNan::new(r.last_price)?,
            previous_close_price: NotNan::new(r.previous_close_price)?,
            underlying_price: r.underlying_price.map(NotNan::new).transpose()?,
            settlement_price: r.settlement_price.map(NotNan::new).transpose()?,
            open_interest: r.open_interest,
            change_in_open_interest: r.change_in_open_interest,
            total_traded_volume: r.total_traded_volume,
            total_traded_value: NotNan::new(r.total_traded_value)?,
            total_number_of_trades: r.total_number_of_trades,
            session_id: r.session_id,
            market_lot_size: r.market_lot_size,
            remarks: r.remarks,
        })
    }
}

#[instrument(skip_all)]
pub async fn fetch_bhav_file(source: &InstrumentSource, date: &NaiveDate) -> Result<Option<Bytes>> {
    let url = get_bhav_file_url(source, date);
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
    let csv_bytes = if bytes.starts_with(b"PK") { unzip_file(&bytes)? } else { bytes };

    Ok(Some(Bytes::from(csv_bytes)))
}

pub fn parse_bhav_csv_records(bytes: Bytes) -> Result<BTreeSet<BhavRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(bytes.reader());

    let headers = rdr.headers()?.iter().take_while(|h| !h.is_empty()).collect();
    rdr.set_headers(headers);

    let prices = rdr
        .deserialize::<BhavRawRecord>()
        .filter_map(|r| r.ok().and_then(|x| BhavRecord::from_raw(x, InstrumentSource::NSE).ok()))
        .collect::<BTreeSet<_>>();

    Ok(prices)
}

/// https://gist.github.com/bugcy013/060150995c73e1b29f5b14ff785a04e9
fn get_bhav_file_url(source: &InstrumentSource, date: &NaiveDate) -> String {
    const CUT_OFF: NaiveDate = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
    const NSE_BASE: &str = "https://nsearchives.nseindia.com/content";

    match (source, date > &CUT_OFF) {
        (InstrumentSource::NSE, true) => {
            let ymd = date.format("%Y%m%d").to_string();
            format!("{NSE_BASE}/cm/BhavCopy_NSE_CM_0_0_0_{ymd}_F_0000.csv.zip")
        }

        (InstrumentSource::NSE, false) => {
            let year = date.format("%Y").to_string();
            let month = date.format("%b").to_string().to_uppercase();
            let date_str = date.format("%d%b%Y").to_string().to_uppercase();
            format!("{NSE_BASE}/historical/EQUITIES/{year}/{month}/cm{date_str}bhav.csv.zip")
        }
    }
}

fn unzip_file(zip_bytes: &[u8]) -> Result<Vec<u8>> {
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
