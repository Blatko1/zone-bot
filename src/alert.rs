use std::time::Instant;

/// Alert which holds information about the time it ocurred,
/// suggested position and other important info.
#[derive(Debug)]
pub struct ZoneAlert {
    time_created: Instant,
    price: String,
    buy_sell: String
}

impl Alert for ZoneAlert {
    fn elapsed_time(&self) -> u64 {
        self.time_created.elapsed().as_secs()
    }

    fn text(&self) -> Vec<Spans> {
        todo!()
    }
    
}

/// Multiple different alerts are available for different causes.
pub trait Alert {
    fn elapsed_time(&self) -> u64;
    fn text(&self) -> Vec<Spans>;
}