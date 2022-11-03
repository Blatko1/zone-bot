use crate::bot::PriceLevel;

pub struct ZoneStrat {
    zones: Vec<Zone>,
    closest_upper: PriceLevel,
    closest_bottom: PriceLevel
}

impl ZoneStrat {
    fn from_zones(zones: Vec<Zone>) -> Self {
        Self { zones, closest_bottom: PriceLevel::ZERO, closest_upper: PriceLevel::ZERO }
    }
}

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

pub trait Strategy {

}