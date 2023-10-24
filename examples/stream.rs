use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use mini_alpaca::stock;
use mini_alpaca::Endpoint;
use mini_alpaca::Result;
use mini_alpaca::StockClient;
use tokio::sync::broadcast::channel;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx_stock, mut rx_stock) = channel::<stock::Event>(10_000);

    // TODO: don't use a loop around a select!
    tokio::spawn(async move {
        loop {
            tokio::select! {
                stock_event = rx_stock.recv() => {
                    // _ = stock_event;
                    match stock_event {
                        Ok(msg) => println!("stock - {msg:?}"),
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
        tx_stock.send(event).unwrap();
        Ok(())
    };

    let mut stock_client: StockClient<stock::Event> =
        StockClient::new(Endpoint::StocksProductionSip, handler).await?;
    stock_client
        .subscribe(stock::Subscribe {
            // trades: Some(vec!["*".into()]),
            quotes: Some(vec!["*".into()]),
            // bars: Some(vec!["*".into()]),
            // daily_bars: Some(vec!["*".into()]),
            // updated_bars: Some(vec!["*".into()]),
            // statuses: Some(vec!["*".into()]),
            // lulds: Some(vec!["*".into()]),
            ..Default::default()
        })
        .await?;

    let run = AtomicBool::new(true);
    tokio::select! {
        s = stock_client.event_loop(&run) => {return s;}
        _ = tokio::signal::ctrl_c() => {
            println!("closing down program");
        }
        // kill the event_loop after a few seconds
        _ = async {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            run.store(false, Ordering::Relaxed);
        } => {}
    }

    Ok(())
}
