use std::sync::atomic::{AtomicBool, Ordering};

use crate::{stock, Endpoint};

pub struct StockClient<'a, T> {
    inner: crate::clients::Client,
    handler: Box<dyn Fn(T) -> crate::Result<()> + 'a + Send>,
}

impl<'a, T: serde::de::DeserializeOwned> StockClient<'a, T> {
    pub async fn new<Callback>(
        endpoint: Endpoint,
        handler: Callback,
    ) -> crate::Result<StockClient<'a, T>>
    where
        Callback: Fn(T) -> crate::Result<()> + 'a + Send,
    {
        let inner = crate::clients::Client::connect(endpoint).await?;
        Ok(StockClient {
            inner,
            handler: Box::new(handler),
        })
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
                let events: Vec<T> = serde_json::from_str(&message)?;
                for event in events {
                    (self.handler)(event)?;
                }
            }
        }
        Ok(())
    }
}
