pub mod crypto;
pub mod endpoint;
pub mod news;
pub mod stock;
pub mod websocket;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
