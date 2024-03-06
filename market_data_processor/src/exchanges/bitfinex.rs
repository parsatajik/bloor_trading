use crate::common::market_data_processor::MarketDataProcessor;
use futures_util::{future::BoxFuture, stream::SplitStream, SinkExt, StreamExt};
use reqwest::Url;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

pub struct Bitfinex {
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Bitfinex {
    pub fn new() -> Self {
        Bitfinex { ws_stream: None }
    }
}

impl MarketDataProcessor for Bitfinex {
    fn connect<'a>(&'a mut self) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let url = Url::parse("wss://api-pub.bitfinex.com/ws/2").unwrap();
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            println!("Connected to the Bitfinex WebSocket feed.");
            self.ws_stream = Some(ws_stream);
        })
    }

    fn subscribe<'a>(&'a mut self, symbols: Vec<String>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(ws_stream) = &mut self.ws_stream {
                for symbol in symbols {
                    let subscribe_message = serde_json::json!({
                        "event": "subscribe",
                        "channel": "ticker",
                        "symbol": symbol
                    });

                    ws_stream
                        .send(Message::Text(subscribe_message.to_string()))
                        .await
                        .expect("Failed to subscribe");
                }
                println!("Subscription messages sent.");
            }
        })
    }

    fn process_message<'a>(&'a mut self) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(ws_stream) = &mut self.ws_stream {
                let (mut write, mut read) = ws_stream.split();

                while let Some(message) = read.next().await {
                    match message {
                        Ok(msg) => {
                            if let Message::Text(text) = msg {
                                // Bitfinex sends updates in arrays, so parsing is different
                                if text.starts_with('[') {
                                    let parsed: Value = serde_json::from_str(&text)
                                        .expect("Failed to parse message");
                                    // Example: print the first value of the array
                                    // https://docs.bitfinex.com/reference/ws-public-ticker
                                    if parsed.is_array() {
                                        println!("Bitfinex Tick Info");
                                        println!("Channel ID: {}", parsed[0].as_u64().unwrap_or(0));
                                        println!("Bid: {}", parsed[1][0].as_f64().unwrap_or(0.0));
                                        println!("Ask: {}", parsed[1][2].as_f64().unwrap_or(0.0));
                                        println!("Price: {}", parsed[1][6].as_f64().unwrap_or(0.0));
                                        println!();
                                    }
                                }
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
