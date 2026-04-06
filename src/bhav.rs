use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Bhav {
    pub prices: Vec<BhavRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BhavRecord {
    #[serde(rename = "TradDt", alias = "TIMESTAMP")]
    pub trade_date: String,
    #[serde(rename = "BizDt")]
    pub business_date: Option<String>,
    #[serde(rename = "Sgmt")]
    pub segment: Option<String>,
    #[serde(rename = "Src")]
    pub source: Option<String>,
    #[serde(rename = "FinInstrmTp", alias = "INSTRUMENT")]
    pub instrument_type: Option<InstrumentType>,
    #[serde(rename = "FinInstrmId")]
    pub instrument_id: Option<String>,
    #[serde(rename = "ISIN")]
    pub isin: Option<String>,
    #[serde(rename = "TckrSymb", alias = "SYMBOL")]
    pub ticker_symbol: String,
    #[serde(rename = "SctySrs", alias = "SERIES")]
    pub security_series: Option<String>,
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
    pub open_price: Option<f64>,
    #[serde(rename = "HghPric", alias = "HIGH")]
    pub high_price: Option<f64>,
    #[serde(rename = "LwPric", alias = "LOW")]
    pub low_price: Option<f64>,
    #[serde(rename = "ClsPric", alias = "CLOSE")]
    pub close_price: Option<f64>,
    #[serde(rename = "LastPric", alias = "LAST")]
    pub last_price: Option<f64>,
    #[serde(rename = "PrvsClsgPric", alias = "PREVCLOSE")]
    pub previous_close_price: Option<f64>,
    #[serde(rename = "UndrlygPric")]
    pub underlying_price: Option<f64>,
    #[serde(rename = "SttlmPric", alias = "SETTLE_PR")]
    pub settlement_price: Option<f64>,
    #[serde(rename = "OpnIntrst", alias = "OPEN_INT")]
    pub open_interest: Option<u64>,
    #[serde(rename = "ChngInOpnIntrst", alias = "CHG_IN_OI")]
    pub change_in_open_interest: Option<i64>,
    #[serde(rename = "TtlTradgVol", alias = "TOTTRDQTY", alias = "CONTRACTS")]
    pub total_traded_volume: Option<u64>,
    #[serde(rename = "TtlTrfVal", alias = "TOTTRDVAL", alias = "VAL_INLAKH")]
    pub total_traded_value: Option<f64>,
    #[serde(rename = "TtlNbOfTxsExctd", alias = "TOTALTRADES")]
    pub total_number_of_trades: Option<u64>,
    #[serde(rename = "SsnId")]
    pub session_id: Option<String>,
    #[serde(rename = "NewBrdLotQty")]
    pub market_lot_size: Option<u64>,
    #[serde(rename = "Rmks")]
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "CUR")]
    Currency,
    #[serde(rename = "CDF")]
    CurrencyFutures,
    #[serde(rename = "CDO")]
    CurrencyOptions,
    #[serde(rename = "IRF")]
    InterestRateFuturesMiborGsec,
    #[serde(rename = "IRT")]
    InterestRateFuturesTbill,
    #[serde(rename = "IRO")]
    InterestRateOptions,
    #[serde(rename = "STK")]
    Stock,
    #[serde(rename = "COM")]
    Commodity,
    #[serde(rename = "COF")]
    CommodityFutures,
    #[serde(rename = "COO")]
    CommodityOptions,
    #[serde(rename = "FUO")]
    OptionsOnFutures,
    #[serde(rename = "STF")]
    StockFutures,
    #[serde(rename = "STO")]
    StockOptions,
    #[serde(rename = "IDF")]
    IndexFutures,
    #[serde(rename = "IDO")]
    IndexOptions,
    #[serde(other)]
    Unknown,
}
