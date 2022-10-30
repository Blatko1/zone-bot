mod input;
mod console;
mod save;
mod zone;
mod tracker;
mod ui;

use binance::api::Binance;
use binance::market::Market;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use console::Console;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{self, Backend},
    Terminal,
};
use zone::ZoneManager;

use crate::console::InputMode;

fn main_loop<B: Backend>(mut console: Console<B>, zones: ZoneManager) {
    // Market
    const SYMBOL: &str = "ETHUSDT";
    let market = Market::new(None, None);

    // In milliseconds:
    const EVENT_INTERVAL: u64 = 2 * 1000;
    const EVENT_DURATION: Duration = Duration::from_millis(EVENT_INTERVAL);

    let mut last = Instant::now();
    loop {
        console.render_ui();

        let elapsed = last.elapsed();
        let timeout = EVENT_DURATION
            .checked_sub(elapsed)
            .unwrap_or(Duration::ZERO);

        match poll_events(timeout) {
            Ok(Some(event)) => {
                match event {
                    event::Event::FocusGained => (),
                    event::Event::FocusLost => (),
                    event::Event::Key(event) => match console.input_mode() {
                        InputMode::Editing => console.process_editing(event),
                        InputMode::Control => console.process_controls(event),
                    },
                    _ => unreachable!(),
                }
                if console.should_exit() {
                    break;
                }
            }
            Ok(None) => (),
            Err(e) => panic!("Terminal Event Error: {e}"),
        }

        // Check the market price
        if elapsed >= EVENT_DURATION {
            //println!("ETHBUSD price: {:?}", market.get_price(SYMBOL).unwrap());
            last = Instant::now();
        }
    }
}

fn poll_events(interval: Duration) -> crossterm::Result<Option<event::Event>> {
    if event::poll(interval)? {
        let event = event::read()?;
        match event {
            Event::FocusGained => return Ok(Some(event)),
            Event::FocusLost => return Ok(Some(event)),
            Event::Key(_) => return Ok(Some(event)),
            _ => (),
        }
    }
    Ok(None)
}

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
    // Market Zones
    let zones = data.get_data();

    // Terminal
    // TODO remove the unwraps and add the "?"
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = backend::CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    // App interface
    let interface = Console::new(terminal);

    main_loop(interface, zones);

    terminal::disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
