use std::{fs, io};

use crate::{ZoneManager, Zone, Priority, PriceLevel};

const SAVE: &str = "bot_data.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    zones: Vec<ZoneData>,
}

impl SaveData {
    fn empty() -> Self {
        Self { zones: Vec::new() }
    }

    pub fn get_data(self) -> ZoneManager {
        let mut zones = Vec::with_capacity(self.zones.len());
        for z in self.zones {
            zones.push(z.into());
        }
        ZoneManager { zones }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ZoneData {
    priority: PriorityData,
    high: PriceLevelData,
    low: PriceLevelData,
}

impl Into<Zone> for ZoneData {
    fn into(self) -> Zone {
        Zone {
            priority: self.priority.into(),
            high: self.high.into(),
            low: self.low.into(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PriceLevelData(f64);

impl Into<PriceLevel> for PriceLevelData {
    fn into(self) -> PriceLevel {
        PriceLevel(self.0)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum PriorityData {
    High = 1,
    Medium = 2,
    Low = 3,
}

impl Into<Priority> for PriorityData {
    fn into(self) -> Priority {
        match self {
            PriorityData::High => Priority::High,
            PriorityData::Medium => Priority::Medium,
            PriorityData::Low => Priority::Low,
        }
    }
}

pub fn load_save() -> io::Result<SaveData> {
    let path = &format!("{}/{}", std::env::current_exe()?.parent().unwrap().to_str().unwrap().trim_start_matches("\\\\?\\"), SAVE);
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
    let path = &format!("{}/{}", std::env::current_exe()?.parent().unwrap().to_str().unwrap(), SAVE);
    let serialized = serde_json::to_string(data).unwrap();

    // Error check in case the file is missing
    match fs::File::open(path) {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!(
                "The save file is missing! \
            Creating a new one with memorized data."
            ),
            _ => return Err(e),
        },
        _ => ()
    }

    fs::write(SAVE, serialized)
}

pub fn new_save() -> io::Result<SaveData> {
    println!("Creating a new save file...");
    let path = &format!("{}/{}", std::env::current_exe()?.parent().unwrap().to_str().unwrap(), SAVE);
    let form = serde_json::to_string(&SaveData::empty()).unwrap();
    fs::write(path, form)?;
    Ok(SaveData::empty())
}
