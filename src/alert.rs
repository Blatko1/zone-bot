use std::time::Instant;

use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
};

use crate::bot::PriceLevel;

/// Alert which holds information about the time it ocurred,
/// suggested position and other important info.
#[derive(Debug)]
pub struct Alert {
    time_created: Instant,
    price: PriceLevel,
    position: Position,
    cause: String,
}

impl Alert {
    fn elapsed_time(&self) -> u64 {
        self.time_created.elapsed().as_secs()
    }

    fn text(&self) -> Vec<Spans> {
        vec![Spans::from(vec![Span::styled(
            "content",
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        )])]
    }
}

#[derive(Debug)]
enum Position {
    Buy,
    Sell,
}
