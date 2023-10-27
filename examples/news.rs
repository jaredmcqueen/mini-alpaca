use std::sync::atomic::AtomicBool;

use mini_alpaca::{Endpoint, NewsClient};
use tokio::sync::mpsc::unbounded_channel;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<()> {
    let (news_tx, mut news_rx) = unbounded_channel::<mini_alpaca::news::Event>();

    tokio::spawn(async move {
        loop {
            while let Some(event) = news_rx.recv().await {
                dbg!(event);
            }
        }
    });

    let handler = |event| {
        news_tx.send(event).unwrap();
        Ok(())
    };

    let mut news_client = NewsClient::new(Endpoint::News, handler).await?;

    news_client
        .subscribe(mini_alpaca::news::Subscribe {
            news: Some(vec!["*".into()]),
        })
        .await?;

    let run = AtomicBool::new(true);
    news_client.event_loop(&run).await?;
    Ok(())
}
