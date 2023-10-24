use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use mini_alpaca::crypto;
use mini_alpaca::crypto::Subscribe;
use mini_alpaca::CryptoClient;
use mini_alpaca::Endpoint;
use mini_alpaca::Result;
use tokio::sync::broadcast::channel;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx_crypto, mut rx_crypto) = channel::<crypto::Event>(10_000);

    // TODO: don't use a loop around a select!
    tokio::spawn(async move {
        loop {
            tokio::select! {
                crypto_event = rx_crypto.recv() => {
                    // _ = crypto_event;
                    match crypto_event {
                        Ok(msg) => println!("crypto - {msg:?}"),
                        Err(e) => {
                            eprintln!("{e:?}");
                            break
                        }
                    }
                }
            }
        }
    });

    let handler = |event| {
        tx_crypto.send(event).unwrap();
        Ok(())
    };

    let mut crypto_client: CryptoClient<crypto::Event> =
        CryptoClient::new(Endpoint::Crypto, handler).await?;
    crypto_client
        .subscribe(Subscribe {
            trades: Some(vec!["*".into()]),
            quotes: Some(vec!["*".into()]),
            bars: Some(vec!["*".into()]),
            updated_bars: Some(vec!["*".into()]),
            dailly_bars: Some(vec!["*".into()]),
            order_books: Some(vec!["*".into()]),
        })
        .await?;

    let run = AtomicBool::new(true);
    tokio::select! {
        s = crypto_client.event_loop(&run) => {return s;}
        _ = tokio::signal::ctrl_c() => {
            println!("closing down program");
        }
        // kill the event_loop after a few seconds
        _ = async {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            run.store(false, Ordering::Relaxed);
        } => {}
    }

    Ok(())
}
