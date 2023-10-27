use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub fn deserialize_datetime_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let str_val = String::deserialize(deserializer)?;
    let datetime: DateTime<Utc> = match DateTime::parse_from_rfc3339(&str_val) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(err) => {
            return Err(serde::de::Error::custom(format!(
                "Invalid datetime: {}",
                err
            )))
        }
    };

    // Convert to nanoseconds
    match datetime.timestamp_nanos_opt() {
        Some(n) => Ok(n as u64),
        None => Err(serde::de::Error::custom("nanoseconds out of range")),
    }
}

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
