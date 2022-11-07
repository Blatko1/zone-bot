use std::{
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
};

use binance::{
    api::Binance, errors::Result as BinanceResult, market::Market,
    model::SymbolPrice,
};

use crate::{
    alert::Alert,
    strategy::{Zone, ZoneStrat},
};

pub struct MarketBot {
    market: Arc<Market>,
    symbol: String,
    price_reader: Receiver<BinanceResult<SymbolPrice>>,
    live_price: PriceLevel,

    zone: ZoneStrat,
    latest_alert: Option<Alert>,

    tick: u16,
}

// TODO replace unwrap() with ?
impl MarketBot {
    const UPDATE_TICKS: u16 = 5;

    pub fn new<I: Into<String>>(symbol: I, zones: Vec<Zone>) -> Self {
        let symbol = symbol.into();
        let market = Arc::new(Market::new(None, None));
        let price_reader = spawn_price_reader(market.clone(), symbol.clone());
        Self {
            market: market,
            symbol,
            price_reader,

            zone: ZoneStrat::from_zones(zones),
            latest_alert: None,

            live_price: PriceLevel::ZERO,
            tick: 0,
        }
    }

    pub fn analyze(&mut self) {}

    pub fn tick(&mut self) {
        if let Some(price) = self.price_reader.try_iter().last() {
            self.live_price = PriceLevel(price.unwrap().price);
        };

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

/// Reading the price from Binance charts blocks the thread for a short period of time
/// which can sometimes delay the user input so a new thread is needed.
///
/// Every [`crate::TICK_INTERVAL`] this thread reads the price which the main thread will
/// read in the next [`crate::TICK_INTERVAL`].
fn spawn_price_reader(
    market: Arc<Market>,
    symbol: String,
) -> Receiver<BinanceResult<SymbolPrice>> {
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        let price = market.get_price(&symbol);
        match tx.send(price) {
            Ok(_) => thread::sleep(crate::TICK_INTERVAL),
            Err(err) => panic!("Thread Error: {err}"),
        }
    });
    rx
}
