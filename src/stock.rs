use crate::common;
use serde::Deserialize;
use serde::Serialize;

// TODO: how much can I move out?

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum StockData {
    Message(Message),
    Event(Event),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum Message {
    #[serde(rename = "auth")]
    Auth(common::Auth),

    #[serde(rename = "subscribe")]
    Subscribe(Subscribe),

    #[serde(rename = "unsubscribe")]
    Unsubscribe(Subscribe),
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Subscribe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "dailyBars")]
    pub daily_bars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedBars")]
    pub updated_bars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lulds: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "T")]
pub enum Event {
    #[serde(rename = "success")]
    Success(common::Success),

    #[serde(rename = "error")]
    Error(common::Error),

    #[serde(rename = "subscription")]
    Subscription(Subscription),

    #[serde(rename = "t")]
    Trade(Trade),

    #[serde(rename = "c")]
    TradeCorrection(TradeCorrection),

    #[serde(rename = "x")]
    TradeCancel(TradeCancel),

    #[serde(rename = "q")]
    Quote(Quote),

    #[serde(rename = "b")]
    Bar(Bar),

    #[serde(rename = "d")]
    DailyBar(Bar),

    #[serde(rename = "u")]
    UpdatedBar(Bar),

    #[serde(rename = "s")]
    Status(Status),

    #[serde(rename = "l")]
    Lulds(Luld),
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Success {
//     #[serde(rename = "msg")]
//     pub message: String,
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Error {
//     #[serde(rename = "msg")]
//     pub message: String,
//
//     #[serde(rename = "code")]
//     pub code: u16,
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subscription {
    #[serde(rename = "trades")]
    pub trades: Vec<String>,

    #[serde(rename = "quotes")]
    pub quotes: Vec<String>,

    #[serde(rename = "bars")]
    pub bars: Vec<String>,

    #[serde(rename = "updatedBars")]
    pub updated_bars: Vec<String>,

    #[serde(rename = "dailyBars")]
    pub daily_bars: Vec<String>,

    #[serde(rename = "statuses")]
    pub statuses: Vec<String>,

    #[serde(rename = "lulds")]
    pub lulds: Vec<String>,

    #[serde(rename = "corrections")]
    pub corrections: Vec<String>,

    #[serde(rename = "cancelErrors")]
    pub cancel_errors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Trade {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub trade_id: u64,

    #[serde(rename = "x")]
    pub exchange_code: String,

    #[serde(rename = "p")]
    pub trade_price: f64,

    #[serde(rename = "s")]
    pub trade_size: u64,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "c")]
    pub trade_condition: Vec<String>,

    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TradeCorrection {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "x")]
    pub exchange_code: String,

    #[serde(rename = "oi")]
    pub original_id: u64,

    #[serde(rename = "op")]
    pub original_price: f64,

    #[serde(rename = "os")]
    pub original_size: u64,

    #[serde(rename = "oc")]
    pub original_conditions: Vec<String>,

    #[serde(rename = "ci")]
    pub corrected_id: u64,

    #[serde(rename = "cp")]
    pub corrected_price: f64,

    #[serde(rename = "cs")]
    pub corrected_size: u64,

    #[serde(rename = "cc")]
    pub corrected_conditions: Vec<String>,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TradeCancel {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub trade_id: u64,

    #[serde(rename = "x")]
    pub exchange_code: String,

    #[serde(rename = "p")]
    pub price: f64,

    #[serde(rename = "s")]
    pub size: u64,

    #[serde(rename = "a")]
    pub action: String,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quote {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "ax")]
    pub ask_exchange_code: String,

    #[serde(rename = "ap")]
    pub ask_price: f64,

    #[serde(rename = "as")]
    pub ask_size: u64,

    #[serde(rename = "bx")]
    pub bid_exchange_code: String,

    #[serde(rename = "bp")]
    pub bid_price: f64,

    #[serde(rename = "bs")]
    pub bid_size: u64,

    #[serde(rename = "c")]
    pub quote_condition: Vec<String>,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bar {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "o")]
    pub open: f64,

    #[serde(rename = "h")]
    pub high: f64,

    #[serde(rename = "l")]
    pub low: f64,

    #[serde(rename = "c")]
    pub close: f64,

    #[serde(rename = "v")]
    pub volume: u64,

    #[serde(rename = "t")]
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Luld {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "u")]
    pub limit_up: f64,

    #[serde(rename = "d")]
    pub limit_down: f64,

    #[serde(rename = "i")]
    pub indicator: String,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Status {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "sc")]
    pub status_code: String,

    #[serde(rename = "sm")]
    pub status_message: String,

    #[serde(rename = "rc")]
    pub reason_code: String,

    #[serde(rename = "rm")]
    pub reason_message: String,

    #[serde(rename = "t")]
    pub timestamp: String,

    #[serde(rename = "z")]
    pub tape: String,
}
