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
    let zones = data.get_data();

    let input_receiver = input_listener_thread();
    
    loop {
        match input_receiver.try_recv() {
            Ok(input) => println!("{input}"),
            Err(mpsc::TryRecvError::Empty) => (),
            Err(e) => panic!("Input Error: {}", e)
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    //let market = Market::new(None, None);
}

pub fn input_listener_thread() -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut buf = String::new();
        loop {
            stdin.read_line(&mut buf).unwrap();
            sender.send(buf.trim().to_string().clone()).unwrap();
            buf.clear();
        }
    });
    receiver
}

#[derive(Debug)]
pub struct ZoneManager {
    zones: Vec<Zone>,
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
