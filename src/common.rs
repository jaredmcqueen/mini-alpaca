use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "T")]
pub enum CommonEvent {
    #[serde(rename = "success")]
    Success(Success),

    #[serde(rename = "error")]
    Error(Error),
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
pub struct Auth {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum CommonMessage {
    #[serde(rename = "auth")]
    Auth(Auth),
}
