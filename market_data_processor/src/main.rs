mod common;
mod exchanges;

use crate::common::market_data_processor::MarketDataProcessor;
use crate::exchanges::coinbase_pro::CoinbasePro;
use crate::exchanges::bitfinex::Bitfinex;
use tokio;

#[tokio::main]
async fn main() {
    let mut coinbase_pro_processor = CoinbasePro::new();
    let mut bitfinex_processor = Bitfinex::new();

    // Connect to Coinbase Pro WebSocket
    coinbase_pro_processor.connect().await;

    // Subscribe to the desired symbols on Coinbase Pro
    let symbols_coinbase = vec!["BTC-USD".to_string()];
    coinbase_pro_processor.subscribe(symbols_coinbase).await;

    // Process incoming messages from Coinbase Pro
    coinbase_pro_processor.process_message().await;

    // Connect to Bitfinex WebSocket
    bitfinex_processor.connect().await;

    // Subscribe to the desired symbols on Bitfinex
    // Note: Bitfinex symbols for trading pairs are prepended with a 't'
    let symbols_bitfinex = vec!["tBTCUSD".to_string()];
    bitfinex_processor.subscribe(symbols_bitfinex).await;

    // Process incoming messages from Bitfinex
    bitfinex_processor.process_message().await;
}