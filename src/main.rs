mod save;

use binance::api::Binance;
use binance::market::Market;
use std::io;

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

    let market = Market::new(None, None);
}

#[derive(Debug)]
struct ZoneManager {
    zones: Vec<Zone>,
}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
struct Zone {
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
struct PriceLevel(f64);

#[derive(Debug, Clone, Copy)]
enum Priority {
    High,
    Medium,
    Low,
}
