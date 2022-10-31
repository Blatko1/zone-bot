use binance::{api::Binance, market::Market};

pub struct MarketTracker {
    market: Market,
    symbol: String,
}

impl MarketTracker {
    pub fn new<S: Into<String>>(symbol: S) -> Self {
        let market = Market::new(None, None);
        Self {
            market,
            symbol: symbol.into(),
        }
    }

    pub fn update(&mut self) {}
}
