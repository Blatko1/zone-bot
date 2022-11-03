use std::{fs, io};

use crate::bot::{Zone, PriceLevel, Priority};

const SAVE: &str = "bot_data.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    zones: Vec<ZoneData>,
}

impl SaveData {
    fn empty() -> Self {
        Self { zones: Vec::new() }
    }

    pub fn data(self) -> Vec<Zone> {
        let mut zones = Vec::with_capacity(self.zones.len());
        for z in self.zones {
            zones.push(z.into());
        }
        zones
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ZoneData {
    priority: PriorityData,
    high: PriceLevelData,
    low: PriceLevelData,
}

impl From<ZoneData> for Zone {
    fn from(data: ZoneData) -> Self {
        Self {
            priority: data.priority.into(),
            high: data.high.into(),
            low: data.low.into(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PriceLevelData(f64);

impl From<PriceLevelData> for PriceLevel {
    fn from(data: PriceLevelData) -> Self {
        Self(data.0)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum PriorityData {
    High = 1,
    Medium = 2,
    Low = 3,
}

impl From<PriorityData> for Priority {
    fn from(data: PriorityData) -> Self {
        match data {
            PriorityData::High => Priority::High,
            PriorityData::Medium => Priority::Medium,
            PriorityData::Low => Priority::Low,
        }
    }
}

pub fn load_save() -> io::Result<SaveData> {
    let path = &format!(
        "{}/{}",
        std::env::current_exe()?
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_start_matches("\\\\?\\"),
        SAVE
    );
    match fs::read(path) {
        Ok(f) => {
            println!("Save file found!");
            match serde_json::from_slice(&f) {
                Ok(data) => Ok(data),
                // TODO implement custom error types
                Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData)),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn save_data(data: &SaveData) -> io::Result<()> {
    let path = &format!(
        "{}/{}",
        std::env::current_exe()?.parent().unwrap().to_str().unwrap(),
        SAVE
    );
    let serialized = serde_json::to_string(data).unwrap();

    // Error check in case the file is missing
    if let Err(e) = fs::File::open(path) {
        match e.kind() {
            io::ErrorKind::NotFound => println!(
                "The save file is missing! \
        Creating a new one with memorized data."
            ),
            _ => return Err(e),
        }
    }

    fs::write(SAVE, serialized)
}

pub fn new_save() -> io::Result<SaveData> {
    println!("Creating a new save file...");
    let path = &format!(
        "{}/{}",
        std::env::current_exe()?.parent().unwrap().to_str().unwrap(),
        SAVE
    );
    let form = serde_json::to_string(&SaveData::empty()).unwrap();
    fs::write(path, form)?;
    Ok(SaveData::empty())
}
