use serde::Deserialize;
use serde::Serialize;

use crate::common::deserialize_datetime_as_u64;
use crate::common::Auth;
use crate::common::Error;
use crate::common::Success;

// [
//     {
//         "T": "n",
//         "id": 35399135,
//         "headline": "RBC Capital Reiterates Outperform on PNC Finl Servs Gr, Maintains $140 Price Target",
//         "summary": "RBC Capital  analyst Gerard Cassidy   reiterates PNC Finl Servs Gr (NYSE:PNC) with a Outperform and maintains $140 price target.",
//         "author": "Benzinga Newsdesk",
//         "created_at": "2023-10-24T16:28:52Z",
//         "updated_at": "2023-10-24T16:28:52Z",
//         "url": "https://www.benzinga.com/news/23/10/35399135/rbc-capital-reiterates-outperform-on-pnc-finl-servs-gr-maintains-140-price-target",
//         "content": "RBC Capital  analyst Gerard Cassidy   reiterates PNC Finl Servs Gr",
//         "symbols": [
//             "PNC"
//         ],
//         "source": "benzinga"
//     }
// ]

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subscription {
    pub news: Vec<String>,
}

// "[{"T":"n","id":35443956,"headline":"EMCOR Group shares are trading higher after the company reported better-than-expected Q3 financial results and raised FY23 revenue guidance above estimates.","summary":"","author":"Benzinga Newsdesk","created_at":"2023-10-26T15:28:26Z","updated_at":"2023-10-26T15:28:26Z","url":"https://www.benzinga.com/wiim/23/10/35443956/emcor-group-shares-are-trading-higher-after-the-company-reported-better-than-expected-q3-financial-r","content":"","symbols":["EME"],"source":"benzinga"}]"
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct News {
    pub id: u64,
    pub headline: String,
    pub summary: String,
    pub author: String,
    #[serde(deserialize_with = "deserialize_datetime_as_u64")]
    pub created_at: u64,
    #[serde(deserialize_with = "deserialize_datetime_as_u64")]
    pub updated_at: u64,
    pub url: String,
    pub content: String,
    pub symbols: Vec<String>,
    pub source: String,
}
