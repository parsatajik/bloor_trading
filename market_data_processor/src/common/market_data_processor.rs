use futures_util::future::BoxFuture;

pub trait MarketDataProcessor {
    // Adjust the connect method to include a lifetime parameter.
    fn connect<'a>(&'a mut self) -> BoxFuture<'a, ()>;

    // The subscribe and process_message methods will also need to be updated to include lifetimes.
    fn subscribe<'a>(&'a mut self, symbols: Vec<String>) -> BoxFuture<'a, ()>;
    fn process_message<'a>(&'a mut self) -> BoxFuture<'a, ()>;
}
