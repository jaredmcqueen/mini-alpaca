use std::sync::atomic::AtomicBool;

use mini_alpaca::{Endpoint, StockClient};
use tokio::sync::mpsc::unbounded_channel;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<()> {
    let (stock_tx, mut stock_rx) = unbounded_channel::<mini_alpaca::stock::Event>();

    tokio::spawn(async move {
        loop {
            while let Some(event) = stock_rx.recv().await {
                dbg!(event);
            }
        }
    });

    let handler = |event| {
        stock_tx.send(event).unwrap();
        Ok(())
    };

    let mut stock_client = StockClient::new(Endpoint::StocksProductionSip, handler).await?;

    stock_client
        .subscribe(mini_alpaca::stock::Subscribe {
            // trades: Some(vec!["TSLA".into()]),
            // quotes: Some(vec!["TSLA".into()]),
            // bars: Some(vec!["TSLA".into()]),
            // daily_bars: Some(vec!["*".into()]),
            updated_bars: Some(vec!["*".into()]),
            // statuses: Some(vec!["*".into()]),
            // lulds: Some(vec!["*".into()]),
            ..Default::default()
        })
        .await?;

    let run = AtomicBool::new(true);
    stock_client.event_loop(&run).await?;
    Ok(())
}
