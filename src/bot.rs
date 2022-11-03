use std::time::Instant;

use binance::{api::Binance, market::Market};
use tui::text::Spans;

pub struct MarketBot<> {
    market: Market,
    symbol: String,
    latest_alert: Option<Box<dyn Alert>>,

    zones: ZoneStrat,

    live_price: PriceLevel,
    tick: u16
}

// TODO replace unwrap() with ?
impl MarketBot {
    const UPDATE_TICKS: u16 = 5;

    pub fn new<S: Into<String>>(symbol: S, zones: Vec<Zone>) -> Self {
        Self {
            market: Market::new(None, None),
            symbol: symbol.into(),
            latest_alert: None,

            zones: ZoneStrat::from_zones(zones),

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

struct ZoneStrat {
    zones: Vec<Zone>,
    closest_upper: PriceLevel,
    closest_bottom: PriceLevel
}

impl ZoneStrat {
    fn from_zones(zones: Vec<Zone>) -> Self {
        Self { zones, closest_bottom: PriceLevel::ZERO, closest_upper: PriceLevel::ZERO }
    }
}

#[derive(Debug)]
pub struct PriceLevel(pub f64);

impl PriceLevel {
    const ZERO: PriceLevel = PriceLevel(0.0);
}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
pub struct Zone {
    pub priority: Priority,
    pub high: PriceLevel,
    pub low: PriceLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// Alert which holds information about the time it ocurred,
/// suggested position and other important info.
#[derive(Debug)]
pub struct ZoneAlert {
    time_created: Instant,
    price: String,
    buy_sell: String
}

impl Alert for ZoneAlert {
    fn elapsed_time(&self) -> u64 {
        self.time_created.elapsed().as_secs()
    }

    fn text(&self) -> Vec<Spans> {
        todo!()
    }
    
}

/// Multiple different alerts are available for different causes.
trait Alert {
    fn elapsed_time(&self) -> u64;
    fn text(&self) -> Vec<Spans>;
}