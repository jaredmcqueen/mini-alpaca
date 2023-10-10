use std::env;
use std::sync::atomic::AtomicBool;

use mini_alpaca::crypto;
use mini_alpaca::crypto::CryptoData;
use mini_alpaca::endpoint::Endpoint;
use mini_alpaca::endpoint::Source;
use mini_alpaca::news;
use mini_alpaca::news::NewsData;
use mini_alpaca::stock;
use mini_alpaca::stock::StockData;
use mini_alpaca::websocket::WebSocketClient;
use mini_alpaca::Result;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<()> {
    let key = env::var("APCA_API_KEY_ID")
        .map_err(|_| "Environment variable APCA_API_KEY_ID not found")?;
    let secret = env::var("APCA_API_SECRET_KEY")
        .map_err(|_| "Environment variable APCA_API_SECRET_KEY_ID not found")?;

    let (tx_stock, mut rx_stock) = channel::<StockData>(10_000);
    let (tx_crypto, mut rx_crypto) = channel::<CryptoData>(10_000);
    let (tx_news, mut rx_news) = channel::<NewsData>(10_000);

    tokio::spawn(async move {
        loop {
            tokio::select! {
                stock_event = rx_stock.recv() => {
                    // _ = stock_event;
                    println!("stock - {stock_event:?}");
                }
                crypto_event = rx_crypto.recv() => {
                    // _ = crypto_event;
                    println!("crypto - {crypto_event:?}");
                }
                news_event = rx_news.recv() => {
                    // _ = news_event;
                    println!("news - {news_event:?}");
                }
            }
        }
    });

    // stock
    let mut stock_client =
        WebSocketClient::new(Endpoint::Stocks(Source::Sip).to_url(), tx_stock).await?;

    let auth = StockData::Message(stock::Message::Auth(stock::Auth {
        key: key.clone(),
        secret: secret.clone(),
    }));
    stock_client.send_message(auth).await?;

    let subscription = StockData::Message(stock::Message::Subscribe(stock::Subscribe {
        trades: Some(vec!["*".into()]),
        quotes: Some(vec!["*".into()]),
        bars: Some(vec!["*".into()]),
        daily_bars: Some(vec!["*".into()]),
        updated_bars: Some(vec!["*".into()]),
        statuses: Some(vec!["*".into()]),
        lulds: Some(vec!["*".into()]),
        // ..Default::default()
    }));
    stock_client.send_message(subscription).await?;

    // crypto
    let mut crypto_client = WebSocketClient::new(Endpoint::Crypto.to_url(), tx_crypto).await?;
    let auth = CryptoData::Message(crypto::Message::Auth(crypto::Auth {
        key: key.clone(),
        secret: secret.clone(),
    }));
    crypto_client.send_message(auth).await?;

    let subscription = CryptoData::Message(crypto::Message::Subscribe(crypto::Subscribe {
        trades: Some(vec!["*".into()]),
        quotes: Some(vec!["*".into()]),
        bars: Some(vec!["*".into()]),
        updated_bars: Some(vec!["*".into()]),
        dailly_bars: Some(vec!["*".into()]),
        order_books: Some(vec!["*".into()]),
        // ..Default::default()
    }));
    crypto_client.send_message(subscription).await?;

    // news
    let mut news_client = WebSocketClient::new(Endpoint::News.to_url(), tx_news).await?;
    let auth = NewsData::Message(news::Message::Auth(news::Auth {
        key: key.clone(),
        secret: secret.clone(),
    }));
    news_client.send_message(auth).await?;

    let subscription = NewsData::Message(news::Message::Subscribe(news::Subscribe {
        news: Some(vec!["*".into()]),
    }));
    news_client.send_message(subscription).await?;

    // run
    let run = AtomicBool::new(true);
    tokio::select! {
        s = stock_client.event_loop(&run) => {return s;}
        c = crypto_client.event_loop(&run) => {return c;}
        n = news_client.event_loop(&run) => {return n;}
        _ = tokio::signal::ctrl_c() => {
            println!("closing down program");
        }
    }
    Ok(())
}
