use std::{io, fs};
use std::path::PathBuf;

use binance::api::Binance;
use binance::market::Market;

const SAVE: &str = "zones.json";

fn main() {
    let data = match load_save() {
        Ok(data) => data,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match new_save() {
                Ok(data) => data,
                Err(e) => panic!("File creation err: {}", e),
            },
            _ => panic!("An error ocurred while parsing the save file!")
        },
    };

    let market = Market::new(None, None);
}

fn load_save() -> io::Result<Vec<u8>> {
    let path = PathBuf::from(".");
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            if path.file_name().unwrap().eq(SAVE) {
                println!("Save file found!");
                return std::fs::read(path);
            }
        }
    }
    println!("Save file not found...");
    io::Result::Err(io::Error::from(io::ErrorKind::NotFound))
}

fn new_save() -> io::Result<Vec<u8>> {
    println!("Creating a new save file.");
    fs::File::create(&format!("./{}", SAVE))?;
    Ok(Vec::new())
}

#[derive(Debug)]
struct ZoneManager {
    zones: Vec<Zone>
}

impl ZoneManager {
    pub fn from_save() -> Self {
        todo!()
    }
}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
struct Zone {
    priority: Priority,
    high: PriceLevel,
    low: PriceLevel
}

impl Zone {
    pub fn new(high: PriceLevel, low: PriceLevel, priority: Priority) -> Self {
        Self { priority, high, low }
    }
}

#[derive(Debug)]
struct PriceLevel(f64);

#[derive(Debug, Clone, Copy)]
enum Priority {
    High,
    Medium,
    Low
}