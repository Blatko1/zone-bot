mod save;

use binance::api::Binance;
use binance::market::Market;
use std::{io, thread, sync::mpsc};

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
    let input_receiver = input_listener_thread();

    let zones = data.get_data();
    const SYMBOL: &str = "ETHUSDT";
    let market = Market::new(None, None);
    
    // In milliseconds:
    const INPUT_INTERVAL: u64 = 100;
    const MARKET_UPDATE_SECONDS: u64 = 10;
    const MARKET_UPDATE_INTERVAL: u64 = MARKET_UPDATE_SECONDS * 1000;
    const CYCLES_PER_UPDATE: u64 = MARKET_UPDATE_INTERVAL / INPUT_INTERVAL;

    let mut update_counter = 0;
    loop {
        match input_receiver.try_recv() {
            Ok(input) => println!("{input}"),
            Err(mpsc::TryRecvError::Empty) => (),
            Err(e) => panic!("Input Error: {}", e)
        }

        // Check the market price
        if update_counter >= CYCLES_PER_UPDATE {
            println!("UPDATE!");
            update_counter = 0;
        }

        update_counter += 1;

        std::thread::sleep(std::time::Duration::from_millis(INPUT_INTERVAL));
    }
}

pub fn input_listener_thread() -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        loop {
            match stdin.read_line(&mut buf) {
                Ok(_) => (),
                Err(e) => panic!("Input Error: {e}"),
            };
            match sender.send(buf.trim().to_string()) {
                Ok(_) => (),
                Err(e) => panic!("Input Sender Error: {e}"),
            };
            buf.clear();
        }
    });
    receiver
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
