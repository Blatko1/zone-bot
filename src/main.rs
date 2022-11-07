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

fn run<B: Backend>(
    mut console: Console<B>,
    mut bot: MarketBot,
) -> Result<(), io::Error> {
    let mut last = Instant::now();

    loop {
        console.render_ui()?;

        let elapsed = last.elapsed();
        let timeout =
            TICK_INTERVAL.checked_sub(elapsed).unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            match event::read()? {
                Event::FocusGained => (),
                Event::FocusLost => (),
                Event::Key(key) => match console.input_mode() {
                    InputMode::Editing => console.process_editing(key),
                    InputMode::Control => console.process_controls(key),
                },
                Event::Resize(..) => {
                    process_resize_batch()?;
                    console.resize();
                }
                _ => (),
            }

            if console.should_exit() {
                break;
            }
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

    Ok(())
}

/// When the user resizes the terminal, resize events come in batches meaning
/// events returned while resizing the window aren't as important as the last
/// resize event giving us the final terminal dimensions.
fn process_resize_batch() -> Result<(), io::Error> {
    while let Ok(true) = event::poll(RESIZE_BATCH_WAIT_DURATION) {
        match event::read()? {
            Event::Resize(..) => (),
            _ => break,
        }
    }
    Ok(())
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

    match run(console, bot) {
        Ok(_) => (),
        Err(err) => panic!("IO Error: {err}"),
    };

    terminal::disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
