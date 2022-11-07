use crate::bot::PriceLevel;

pub struct ZoneStrat {
    zones: Vec<Zone>,
    closest_upper: PriceLevel,
    closest_bottom: PriceLevel,
}

impl ZoneStrat {
    pub fn from_zones(zones: Vec<Zone>) -> Self {
        Self {
            zones,
            closest_bottom: PriceLevel::NAN,
            closest_upper: PriceLevel::NAN,
        }
    }
}

impl Strategy for ZoneStrat {}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
pub struct Zone {
    pub priority: ZonePriority,
    pub high: PriceLevel,
    pub low: PriceLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum ZonePriority {
    High,
    Medium,
    Low,
}

pub trait Strategy {}
