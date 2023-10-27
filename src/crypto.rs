use crate::common::deserialize_datetime_as_u64;
use crate::common::Auth;
use crate::common::Error;
use crate::common::Success;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CryptoData {
    Message(Message),
    Event(Event),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum Message {
    #[serde(rename = "auth")]
    Auth(Auth),

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

    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedBars")]
    pub updated_bars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "dailyBars")]
    pub dailly_bars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "orderbooks")]
    pub order_books: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "T")]
pub enum Event {
    #[serde(rename = "success")]
    Success(Success),

    #[serde(rename = "error")]
    Error(Error),

    #[serde(rename = "subscription")]
    Subscription(Subscription),

    #[serde(rename = "t")]
    Trade(Trade),

    #[serde(rename = "o")]
    OrderBook(OrderBook),

    #[serde(rename = "q")]
    Quote(Quote),
    //
    #[serde(rename = "b")]
    Bar(Bar),

    #[serde(rename = "d")]
    DailyBar(Bar),

    #[serde(rename = "u")]
    UpdatedBar(Bar),
}

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

    #[serde(rename = "orderbooks")]
    pub order_books: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Trade {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price: f64,

    #[serde(rename = "s")]
    pub size: f64,

    #[serde(deserialize_with = "deserialize_datetime_as_u64", rename = "t")]
    pub timestamp: u64,

    #[serde(rename = "i")]
    pub id: u64,

    #[serde(rename = "tks")]
    pub taker_side: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Book {
    #[serde(rename = "p")]
    pub price: f64,

    #[serde(rename = "s")]
    pub size: f64,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderBook {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(deserialize_with = "deserialize_datetime_as_u64", rename = "t")]
    pub timestamp: u64,

    #[serde(rename = "b")]
    pub bid: Vec<Book>,

    #[serde(rename = "a")]
    pub ask: Vec<Book>,

    #[serde(rename = "r")]
    pub reset: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quote {
    #[serde(rename = "S")]
    pub symbol: String,

    #[serde(rename = "bp")]
    pub bid_price: f64,

    #[serde(rename = "bs")]
    pub bid_size: f64,

    #[serde(rename = "ap")]
    pub ask_price: f64,

    #[serde(rename = "as")]
    pub ask_size: f64,

    #[serde(deserialize_with = "deserialize_datetime_as_u64", rename = "t")]
    pub timestamp: u64,
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
    pub volume: f64,

    #[serde(deserialize_with = "deserialize_datetime_as_u64", rename = "t")]
    pub timestamp: u64,

    #[serde(rename = "n")]
    pub num_trades: u64,

    #[serde(rename = "vw")]
    pub volume_weight: f64,
}
