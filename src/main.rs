mod save;

use binance::api::Binance;
use binance::market::Market;
use crossterm::event::{poll, self};
use std::{io, thread, sync::mpsc, time::{Duration, Instant}};

fn main() {
    let data = match save::load_save() {
        Ok(data) => data,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("The save file does not exist!");
                match save::new_save() {
                    Ok(data) => data,
                    Err(e) => panic!("File creation err: {}", e),
                }
            }
            _ => panic!("An error ocurred while parsing the save file: {}", e),
        },
    };
    let zones = data.get_data();
    const SYMBOL: &str = "ETHUSDT";
    let market = Market::new(None, None);
    
    // In milliseconds:
    const EVENT_WAIT_TIME: u64 = 500;
    const MARKET_UPDATE_INTERVAL: u128 = 2 * 1000;
    const EVENT_WAIT_DURATION: Duration = Duration::from_millis(EVENT_WAIT_TIME);
    let mut now = Instant::now();

    loop {

        match process_events(EVENT_WAIT_DURATION) {
            Ok(_) => (),
            Err(e) => panic!("Terminal Event Error: {e}"),
        }

        // Check the market price
        let elapsed = now.elapsed().as_millis();
        if elapsed > MARKET_UPDATE_INTERVAL {
            println!("UPDATE!");
            now = Instant::now();
        }
    }
}

pub fn process_events(interval: Duration) -> crossterm::Result<()> {
    if event::poll(interval)? {
        match event::read()? {
            e => println!("Event found!: {e:?}"),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct ZoneManager {
    zones: Vec<Zone>,
    up_closest: Option<PriceLevel>,
    down_closest: Option<PriceLevel>
}

impl ZoneManager {
    pub fn from_zones(zones: Vec<Zone>) -> Self {
        Self {
            zones,
            up_closest: None,
            down_closest: None,
        }
    }

    pub fn update(&mut self, cmp_price: PriceLevel) {
        
    }
}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
pub struct Zone {
    priority: Priority,
    high: PriceLevel,
    low: PriceLevel,
}

impl Zone {
    pub fn new(high: PriceLevel, low: PriceLevel, priority: Priority) -> Self {
        Self {
            priority,
            high,
            low,
        }
    }
}

#[derive(Debug)]
pub struct PriceLevel(f64);

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}
