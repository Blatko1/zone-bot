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
    // TODO put Symbol struct instead
    symbol: String,
    price_tracker: PriceTracker,

    zone: ZoneStrat,
    latest_alert: Option<Alert>,

    tick: u16,
}

// TODO replace unwrap() with ?
impl MarketBot {
    const UPDATE_TICKS: u16 = 5;

    pub fn new<S: Into<String>>(symbol: S, zones: Vec<Zone>) -> Self {
        let symbol = symbol.into();
        let market = Arc::new(Market::new(None, None));
        let price_tracker = PriceTracker::new(market.clone(), symbol.clone());
        Self {
            market: market,
            symbol,
            price_tracker,

            zone: ZoneStrat::from_zones(zones),
            latest_alert: None,

            tick: 0,
        }
    }

    pub fn analyze(&mut self) {}

    pub fn tick(&mut self) {
        self.price_tracker.track();
        self.tick += 1;

        if self.tick >= Self::UPDATE_TICKS {
            self.analyze();
            self.tick = 0;
        }
    }

    // TODO maybe do inlining
    pub fn get_price(&self) -> PriceLevel {
        self.price_tracker.get_price()
    }
}

struct PriceTracker {
    price: PriceLevel,
    reader: Receiver<BinanceResult<SymbolPrice>>
}

impl PriceTracker {
    fn new(market: Arc<Market>, symbol: String) -> Self {
        let reader = spawn_price_reader(market, symbol);
        Self {
            price: PriceLevel::NAN,
            reader
        }
    }

    fn track(&mut self) {
        if let Some(price) = self.reader.try_iter().last() {
            // TODO add Into impl for PriceLevel from SymbolPrice
            self.price = PriceLevel(price.unwrap().price);
        }
    }

    fn get_price(&self) -> PriceLevel {
        self.price
    }
}

#[derive(Debug)]
struct Symbol(String);

#[derive(Debug, Clone, Copy)]
pub struct PriceLevel(pub f64);

impl PriceLevel {
    pub const NAN: PriceLevel = PriceLevel(f64::NAN);
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
            Err(_) => break,
        }
    });
    rx
}
