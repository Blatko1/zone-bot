#[derive(Debug)]
pub struct ZoneManager {
    zones: Vec<Zone>,
    up_closest: Option<PriceLevel>,
    down_closest: Option<PriceLevel>,
}

impl ZoneManager {
    pub fn from_zones(zones: Vec<Zone>) -> Self {
        Self {
            zones,
            up_closest: None,
            down_closest: None,
        }
    }

    pub fn update(&mut self, cmp_price: PriceLevel) {}
}

/// Represents a "resistance" or a "support" zone with the `high` and the `low` limit.
/// Priority represents the credibility of each zone.
#[derive(Debug)]
pub struct Zone {
    pub priority: Priority,
    pub high: PriceLevel,
    pub low: PriceLevel,
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
pub struct PriceLevel(pub f64);

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}
