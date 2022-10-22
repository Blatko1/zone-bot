mod input;
mod save;
mod zone;

use binance::api::Binance;
use binance::market::Market;
use crossterm::{
    cursor,
    event::{self, poll, KeyEvent},
    terminal,
};
use std::{
    fmt::Write,
    io::{self},
    str::CharIndices,
    time::{Duration, Instant},
};
use tui::{
    backend::{self, Backend},
    Terminal,
};
use zone::ZoneManager;

use crate::input::{InputHandler, Interruption};

pub struct Interface<B: Backend> {
    terminal: Terminal<B>,
    input_mode: InputMode,
}

impl<B: Backend> Interface<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            input_mode: InputMode::Editing,
        }
    }

    pub fn process_events(&mut self, event: KeyEvent) {}

    pub fn process_interruption(&mut self, interruption: Interruption) {
        match interruption {
            Interruption::Enter(buf) => println!("You entered: {buf}"),
            Interruption::Esc => (),
        }
    }
}

pub enum InputMode {
    Editing,
    Control,
}

fn main_loop<B: Backend>(mut intf: Interface<B>, zones: ZoneManager) {
    // Market
    const SYMBOL: &str = "ETHUSDT";
    let market = Market::new(None, None);

    // In milliseconds:
    const EVENT_INTERVAL: u64 = 2 * 1000;
    const EVENT_DURATION: Duration = Duration::from_millis(EVENT_INTERVAL);

    let mut input = InputHandler::new();
    let mut last = Instant::now();
    loop {
        let elapsed = last.elapsed();
        let timeout = EVENT_DURATION
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::ZERO);

        match poll_events(timeout) {
            Ok(Some(event)) => match event {
                event::Event::FocusGained => (),
                event::Event::FocusLost => (),
                event::Event::Key(event) => match intf.input_mode {
                    InputMode::Editing => match input.editing_event(event) {
                        Some(intr) => intf.process_interruption(intr),
                        None => println!("input: {}", input),
                    },
                    InputMode::Control => intf.process_events(event),
                },
                _ => unreachable!(),
            },
            Ok(None) => (),
            Err(e) => panic!("Terminal Event Error: {e}"),
        }

        // Check the market price
        if elapsed >= EVENT_DURATION {
            //println!("UPDATE!");
            last = Instant::now();
        }
    }
}

fn poll_events(interval: Duration) -> crossterm::Result<Option<event::Event>> {
    if event::poll(interval)? {
        match event::read()? {
            event => match event {
                event::Event::FocusGained => return Ok(Some(event)),
                event::Event::FocusLost => return Ok(Some(event)),
                event::Event::Key(_) => return Ok(Some(event)),
                _ => (),
            },
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
    // Terminal
    let stdout = io::stdout();
    let backend = backend::CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    // App interface
    let interface = Interface::new(terminal);

    // Market Zones
    let zones = data.get_data();

    terminal::enable_raw_mode().unwrap();
    main_loop(interface, zones);
}
