pub mod clients;
pub use clients::{Client, CryptoClient, NewsClient, StockClient};

pub mod endpoint;
pub use endpoint::Endpoint;

pub mod common;
pub mod crypto;
pub mod news;
pub mod stock;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
