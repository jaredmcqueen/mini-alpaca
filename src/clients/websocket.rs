use std::env;

use crate::common;
use crate::common::CommonEvent;
use crate::Endpoint;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct Client {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    pub async fn connect(endpoint: Endpoint) -> crate::Result<Client> {
        let (stream, _) = connect_async(endpoint.to_url()).await?;

        let mut client = Client { stream };
        let message = client.get_next_message().await?.unwrap();
        client.get_response(message)?;

        match endpoint {
            Endpoint::StocksProductionSip
            | Endpoint::StocksSandboxSip
            | Endpoint::Crypto
            | Endpoint::News => {
                client.authenticate().await?;
            }
            _ => {
                // TODO: better logging
                println!("no authentication needed");
            }
        }

        let message = client.get_next_message().await?.unwrap();
        client.get_response(message)?;

        Ok(client)
    }

    fn get_response(&self, message: String) -> crate::Result<()> {
        let events: Vec<crate::common::CommonEvent> = serde_json::from_str(&message)?;

        match events.get(0).unwrap() {
            CommonEvent::Error(msg) => {
                // TODO: better logging
                eprintln!("error: {}", msg.message);
                Err("error".into())
            }
            CommonEvent::Success(msg) => {
                // TODO: better logging
                println!("succsss {}", msg.message);
                Ok(())
            }
        }
    }

    pub async fn authenticate(&mut self) -> crate::Result<()> {
        let key = env::var("APCA_API_KEY_ID")
            .map_err(|_| "Environment variable APCA_API_KEY_ID not found")?;
        let secret = env::var("APCA_API_SECRET_KEY")
            .map_err(|_| "Environment variable APCA_API_SECRET_KEY_ID not found")?;

        let auth_message = common::CommonMessage::Auth(common::Auth { key, secret });
        let json = serde_json::to_string(&auth_message).map_err(|e| {
            dbg!(&e);
            dbg!(&auth_message);
            "failed to serialize message"
        })?;
        self.stream
            .send(Message::Text(json))
            .await
            .map_err(|e| e.into())
    }

    /// returns the raw text of the next message
    pub async fn get_next_message(&mut self) -> crate::Result<Option<String>> {
        let message = self.stream.next().await.unwrap()?;
        match message {
            Message::Text(msg) => {
                if msg.is_empty() {
                    return Ok(None);
                }
                Ok(Some(msg))
            }
            Message::Close(_) => Err("websocket closed".into()),
            // TODO: add ping/pong response?
            _ => Ok(None),
        }
    }

    pub async fn send_message(&mut self, message: String) -> crate::Result<()> {
        self.stream.send(Message::Text(message)).await?;
        Ok(())
    }
}
