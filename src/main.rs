use binance::api::Binance;
use binance::market::Market;

fn main() {
    let market = Market::new(None, None);


}

#[derive(Debug)]
struct ZoneManager {
    zones: Vec<Zone>
}

impl ZoneManager {

}

#[derive(Debug)]
struct Zone {
    priority: Priority,
    up_limit: Price,
    down_limit: Price
}

#[derive(Debug)]
struct Price(f64);

#[derive(Debug)]
enum Priority {
    High,
    Medium,
    Low
}