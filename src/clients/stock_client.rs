use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::broadcast::Sender;

use crate::{stock, Endpoint};

pub struct StockClient {
    inner: crate::clients::Client,
    sender: Sender<stock::Event>,
}

impl StockClient {
    pub async fn connect(
        endpoint: Endpoint,
        sender: Sender<stock::Event>,
    ) -> crate::Result<StockClient> {
        let inner = crate::clients::Client::connect(endpoint).await?;
        Ok(StockClient { inner, sender })
    }

    async fn serialize_and_send(&mut self, message: stock::Message) -> crate::Result<()> {
        let json = serde_json::to_string(&message).map_err(|e| {
            dbg!(&e);
            dbg!(&message);
            "failed to serialize message"
        })?;
        self.inner.send_message(json).await
    }

    pub async fn subscribe(&mut self, subscription: crate::stock::Subscribe) -> crate::Result<()> {
        let subscription = stock::Message::Subscribe(subscription);
        self.serialize_and_send(subscription).await
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> crate::Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(message) = self.inner.get_next_message().await? {
                let events: Vec<stock::Event> = serde_json::from_str(&message)?;
                for event in events {
                    self.sender.send(event)?;
                }
            }
        }
        Ok(())
    }
}
