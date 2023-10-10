use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum NewsData {
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Auth {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Subscribe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news: Option<Vec<String>>,
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

    #[serde(rename = "n")]
    News(News),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {
    #[serde(rename = "msg")]
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    #[serde(rename = "msg")]
    pub message: String,

    #[serde(rename = "code")]
    pub code: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subscription {
    pub news: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct News {
    pub id: u64,
    pub headline: String,
    pub summary: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub content: String,
    pub symbols: Vec<String>,
    pub source: String,
}
