use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::Result;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct WebSocketClient<T> {
    pub data_channel: Sender<T>,
    pub ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl<T: DeserializeOwned + Serialize + Send + 'static + std::fmt::Debug> WebSocketClient<T> {
    pub async fn new(url: String, data_channel: Sender<T>) -> crate::Result<Self> {
        let (ws_stream, _) = connect_async(url).await?;
        let client = Self {
            data_channel,
            ws_stream,
        };
        Ok(client)
    }

    pub async fn send_message(&mut self, message: T) -> Result<()> {
        let json = serde_json::to_string(&message).map_err(|e| {
            dbg!(e);
            dbg!(message);
            "failed to serialize message"
        })?;

        match self.ws_stream.send(Message::Text(json)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("error sending message: {e}").into()),
        }
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            let ws_message = if let Some(Ok(message)) = self.ws_stream.next().await {
                message
            } else {
                return Err("error".into());
            };

            match ws_message {
                Message::Text(raw_string) if !raw_string.is_empty() => {
                    // dbg!(&raw_string);
                    let events: Vec<T> = serde_json::from_str(&raw_string).map_err(|e| {
                        dbg!(e);
                        dbg!(raw_string);
                        println!("The type of T is: {}", std::any::type_name::<T>());
                        "failed to parse json"
                    })?;

                    for event in events {
                        self.data_channel
                            .send(event)
                            .await
                            .map_err(|_| "failed to send")?
                    }
                }
                Message::Ping(p) => self
                    .ws_stream
                    .send(Message::Pong(p))
                    .await
                    .expect("failed to send pong message"),
                Message::Close(_) => {
                    return Err("Disconnected".into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}
