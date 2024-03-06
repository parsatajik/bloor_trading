use crate::common::market_data_processor::MarketDataProcessor;
use futures_util::{future::BoxFuture, stream::SplitStream, SinkExt, StreamExt};
use reqwest::Url;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

pub struct CoinbasePro {
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl CoinbasePro {
    pub fn new() -> Self {
        CoinbasePro { ws_stream: None }
    }
}

impl MarketDataProcessor for CoinbasePro {
    fn connect<'a>(&'a mut self) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let url = Url::parse("wss://ws-feed.exchange.coinbase.com").unwrap();
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            println!("Connected to the Coinbase Pro WebSocket feed.");
            self.ws_stream = Some(ws_stream);
        })
    }

    fn subscribe<'a>(&'a mut self, symbols: Vec<String>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(ws_stream) = &mut self.ws_stream {
                let subscribe_message = serde_json::json!({
                    "type": "subscribe",
                    "product_ids": symbols,
                    "channels": ["ticker"]
                });

                ws_stream
                    .send(Message::Text(subscribe_message.to_string()))
                    .await
                    .expect("Failed to subscribe");
                println!("Subscription message sent.");
            }
        })
    }

    fn process_message<'a>(&'a mut self) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(ws_stream) = &mut self.ws_stream {
                // Correctly use the split method directly on the WebSocketStream instance.
                let (mut write, mut read) = ws_stream.split();

                while let Some(message) = read.next().await {
                    match message {
                        Ok(msg) => {
                            if let Message::Text(text) = msg {
                                let parsed: Value =
                                    serde_json::from_str(&text).expect("Failed to parse message");
                                println!("Coinbase Pro Tick Info");
                                println!("price: {}", parsed["price"].as_str().unwrap_or("N/A"));
                                println!(
                                    "best_bid: {}",
                                    parsed["best_bid"].as_str().unwrap_or("N/A")
                                );
                                println!(
                                    "best_ask: {}",
                                    parsed["best_ask"].as_str().unwrap_or("N/A")
                                );
                                println!();
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
        })
    }
}
