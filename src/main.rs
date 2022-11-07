mod alert;
mod bot;
mod console;
mod input;
mod save;
mod strategy;
mod ui;

use bot::MarketBot;
use console::Console;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{self, Backend},
    Terminal,
};

use crate::console::InputMode;

const DEFAULT_SYMBOL: &str = "ETHUSDT";
const TICK_INTERVAL: Duration = Duration::from_millis(2000);
const RESIZE_BATCH_WAIT_DURATION: Duration = Duration::from_millis(100);

fn run<B: Backend>(mut console: Console<B>, mut bot: MarketBot) {
    let mut last = Instant::now();
    loop {
        match console.render_ui() {
            Ok(_) => (),
            Err(err) => panic!("Terminal Render Error: {err}"),
        };

        let elapsed = last.elapsed();
        let timeout =
            TICK_INTERVAL.checked_sub(elapsed).unwrap_or(Duration::ZERO);

        match poll_events(timeout) {
            Ok(Some(event)) => {
                match event {
                    Event::FocusGained => (),
                    Event::FocusLost => (),
                    Event::Key(event) => match console.input_mode() {
                        InputMode::Editing => console.process_editing(event),
                        InputMode::Control => console.process_controls(event),
                    },
                    Event::Resize(..) => {
                        process_resize_batch();
                        console.resize()
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
            //println!("{}", elapsed.as_millis());
            last = Instant::now();

            // Tick the bot. Every tick update the live price
            // and every 5 ticks bot analyzes the price.
            bot.tick();

            // Update the UI with fresh market data.
            console.update_ui(&bot);
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
            Event::Resize(..) => return Ok(Some(event)),
            _ => (),
        }
    }
    Ok(None)
}

/// When the user resizes the terminal, resize events come in batches meaning 
/// events returned while resizing the window aren't as important as the last 
/// resize event giving us the final terminal dimensions.
fn process_resize_batch() {
    while let Ok(true) = event::poll(RESIZE_BATCH_WAIT_DURATION) {
        match event::read().unwrap() {
            Event::Resize(..) => (),
            _ => break
        }
    }
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

    // Stdout
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

    run(console, bot);

    terminal::disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
