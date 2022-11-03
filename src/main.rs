mod console;
mod input;
mod save;
mod bot;
mod ui;
mod strategy;
mod alert;

use console::Console;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use strategy::Strategy;
use std::{
    io,
    time::{Duration, Instant},
};
use bot::MarketBot;
use tui::{
    backend::{self, Backend},
    Terminal,
};

use crate::console::InputMode;

const DEFAULT_SYMBOL: &str = "ETHUSDT";
const TICK_INTERVAL: Duration = Duration::from_millis(1500);

fn main_loop<B: Backend>(mut console: Console<B>, bot: MarketBot<dyn Strategy>) {
    let mut last = Instant::now();
    loop {
        match console.render_ui() {
            Ok(_) => (),
            Err(err) => panic!("Terminal Render Error: {err}"),
        };

        let elapsed = last.elapsed();
        let timeout = TICK_INTERVAL
            .checked_sub(elapsed)
            .unwrap_or(Duration::ZERO);

        match poll_events(timeout) {
            Ok(Some(event)) => {
                match event {
                    Event::FocusGained => (),
                    Event::FocusLost => (),
                    Event::Key(event) => match console.input_mode() {
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

        // One tick happens every second. 1 tick == 1 second
        if elapsed >= TICK_INTERVAL {
            println!("{}", elapsed.as_millis());
            last = Instant::now();

            // Tick the bot. Every tick update the live price
            // and every 5 ticks bot analyzes the price.
            bot.tick();
        }
    }
}

fn poll_events(interval: Duration) -> crossterm::Result<Option<event::Event>> {
    if event::poll(interval)? {
        let event = event::read()?;
        match event {
            Event::FocusGained => return Ok(Some(event)),
            Event::FocusLost => return Ok(Some(event)),
            Event::Key(..) => return Ok(Some(event)),
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
    let zones = data.data();

    // Terminal
    // TODO remove the unwraps and add the "?"
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // Console
    let backend = backend::CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    let console = Console::new(terminal);

    // Market
    let bot = MarketBot::new(DEFAULT_SYMBOL, zones);

    main_loop(console, bot);

    terminal::disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
