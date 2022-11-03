use binance::{api::Binance, market::Market};

use crate::{alert::Alert, strategy::{Strategy, ZoneStrat}};

pub struct MarketBot<S: Strategy> {
    market: Market,
    symbol: String,
    latest_alert: Option<Box<dyn Alert>>,

    strategies: Vec<S>,
    live_price: PriceLevel,
    
    tick: u16
}

// TODO replace unwrap() with ?
impl<S: Strategy> MarketBot<S> {
    const UPDATE_TICKS: u16 = 5;

    pub fn new<S: Into<String>>(symbol: S, zones: Vec<Zone>) -> Self {
        Self {
            market: Market::new(None, None),
            symbol: symbol.into(),
            latest_alert: None,

            strategies: vec![ZoneStrat::from_zones(zones)],

            live_price: PriceLevel::ZERO,
            tick: 0
        }
    }

    pub fn analyze(&mut self) {

    }

    pub fn tick(&mut self) {
        self.live_price = PriceLevel(self.market.get_price(&self.symbol).unwrap().price);
        self.tick += 1;

        if self.tick >= Self::UPDATE_TICKS {
            self.analyze();
            self.tick = 0;
        }
    }
}

#[derive(Debug)]
pub struct PriceLevel(pub f64);

impl PriceLevel {
    pub const ZERO: PriceLevel = PriceLevel(0.0);
}