pub mod clients;
pub use clients::{Client, StockClient};

pub mod endpoint;
pub use endpoint::Endpoint;

pub mod common;
pub mod stock;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
