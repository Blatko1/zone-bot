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
use tui::{backend, Terminal};

pub struct App {
    input_mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Editing,
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
    let mut stdout = io::stdout();
    let backend = backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    let zones = data.get_data();
    const SYMBOL: &str = "ETHUSDT";
    let market = Market::new(None, None);

    // In milliseconds:
    const EVENT_INTERVAL: u64 = 2 * 1000;
    const EVENT_DURATION: Duration = Duration::from_millis(EVENT_INTERVAL);

    let mut last = Instant::now();
    let mut input = InputHandler::new();
    terminal::enable_raw_mode().unwrap();
    loop {
        let elapsed = last.elapsed();
        let timeout = EVENT_DURATION
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::ZERO);

        match poll_events(timeout) {
            Ok(Some(event)) => match event {
                event::Event::FocusGained => (),
                event::Event::FocusLost => (),
                event::Event::Key(event) => match app.input_mode {
                    InputMode::Editing => match input.editing_event(event) {
                        Some(InputInterruption::Enter(buf)) => {
                            println!("You entered: {buf}")
                        }
                        Some(InputInterruption::Esc) => break,
                        None => println!(
                            "buf: {}, pos: {}, len: {}, char_len: {}",
                            input.buffer,
                            input.get_cursor_position(),
                            input.buffer_len(),
                            input.buffer_char_len()
                        ),
                    },
                    InputMode::Normal => input.normal_event(event),
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

pub fn poll_events(
    interval: Duration,
) -> crossterm::Result<Option<event::Event>> {
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

// TODO
const INPUT_BUFFER_CAPACITY: usize = 10;

/// Handler for inputs trough all [`InputMode`] modes.
struct InputHandler {
    buffer: String,
    /// Preferably used because of chars made from multiple bytes.
    char_count: usize,
    multi_byte_chars: usize,
    cursor: CursorPosition,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(INPUT_BUFFER_CAPACITY),
            char_count: 0,
            multi_byte_chars: 0,
            cursor: CursorPosition::End,
        }
    }

    pub fn normal_event(&mut self, event: KeyEvent) {}

    pub fn editing_event(
        &mut self,
        event: KeyEvent,
    ) -> Option<InputInterruption> {
        let key = event.code;
        match key {
            event::KeyCode::Backspace => self.backspace_key(),
            event::KeyCode::Enter => {
                let buffer = self.buffer.drain(..).collect();
                self.clear();
                return Some(InputInterruption::Enter(buffer));
            }
            event::KeyCode::Left => self.left_key(),
            event::KeyCode::Right => self.right_key(),
            //event::KeyCode::Up => todo!(),
            //event::KeyCode::Down => todo!(),
            event::KeyCode::Home => self.cursor = CursorPosition::Pos(0),
            event::KeyCode::End => self.cursor = CursorPosition::End,
            event::KeyCode::Delete => self.delete_key(),
            event::KeyCode::Char(c) => self.add_char(c),
            // Try exiting the editing mode if there is nothing in the buffer.
            event::KeyCode::Esc => {
                if self.buffer.is_empty() {
                    return Some(InputInterruption::Esc);
                } else {
                    self.clear()
                }
            }
            _ => (),
        };
        None
    }

    fn add_char(&mut self, c: char) {
        self.char_count += 1;
        if c.len_utf8() > 1 {
            self.multi_byte_chars += 1;
        }

        match &mut self.cursor {
            CursorPosition::Pos(index) => {
                if self.multi_byte_chars > 0 {
                    let (byte_pos, _) =
                        self.buffer.char_indices().nth(*index).unwrap();
                    self.buffer.insert(byte_pos, c);
                } else {
                    self.buffer.insert(*index, c);
                }
                // Increment because cursor is being moved to the right after char input
                *index += 1;
            }
            CursorPosition::End => self.buffer.push(c),
        }
    }

    fn left_key(&mut self) {
        match self.cursor {
            CursorPosition::Pos(index) => {
                if index > 0 {
                    self.cursor = CursorPosition::Pos(index - 1)
                }
            }
            CursorPosition::End => {
                if !self.buffer.is_empty() {
                    self.cursor = CursorPosition::Pos(self.char_count - 1)
                }
            }
        }
    }

    fn right_key(&mut self) {
        match self.cursor {
            CursorPosition::Pos(index) => {
                let new_pos = index + 1;
                self.cursor = if new_pos == self.char_count {
                    CursorPosition::End
                } else {
                    CursorPosition::Pos(new_pos)
                }
            }
            // Do nothing since it's already at the end
            CursorPosition::End => (),
        }
    }

    fn delete_key(&mut self) {
        match self.cursor {
            CursorPosition::Pos(index) => {
                if self.multi_byte_chars > 0 {
                    self.remove_char_alter(index);
                } else {
                    self.remove_char(index);
                }

                if self.buffer.is_empty() || index == self.char_count {
                    self.cursor = CursorPosition::End
                }
            }
            // Do nothing since there is no chars after the ending
            CursorPosition::End => (),
        };
    }

    fn backspace_key(&mut self) {
        match self.cursor {
            CursorPosition::Pos(index) => {
                if index > 0 {
                    // Backspace removes the char left from the cursor
                    if self.multi_byte_chars > 0 {
                        self.remove_char_alter(index - 1);
                    } else {
                        self.remove_char(index - 1);
                    }
                    if self.buffer.is_empty() {
                        self.cursor = CursorPosition::End
                    } else {
                        self.left_key()
                    }
                }
            }
            CursorPosition::End => {
                if !self.buffer.is_empty() {
                    if self.multi_byte_chars > 0 {
                        self.buffer.pop();
                        self.multi_byte_chars -= 1;
                        self.char_count -= 1;
                    } else {
                        self.buffer.pop();
                        self.char_count -= 1;
                    }
                }
            }
        }
    }

    // Used when there are multi byte chars in the buffer
    fn remove_char_alter(&mut self, index: usize) {
        assert!(index < self.char_count);
        let byte_pos = self.buffer.char_indices().nth(index).unwrap().0;
        if self.buffer.remove(byte_pos).len_utf8() > 1 {
            self.multi_byte_chars -= 1;
        }
        self.char_count -= 1;
    }

    fn remove_char(&mut self, index: usize) {
        assert!(index < self.char_count);
        self.buffer.remove(index);
        self.char_count -= 1;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.char_count = 0;
        self.multi_byte_chars = 0;
        self.cursor = CursorPosition::End;
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn buffer_char_len(&self) -> usize {
        self.buffer.chars().count()
    }

    pub fn get_cursor_position(&self) -> usize {
        match self.cursor {
            CursorPosition::Pos(i) => i,
            CursorPosition::End => self.char_count,
        }
    }
}

/// Events which occur when the user tries exiting the Editing mode.
enum InputInterruption {
    /// Exit the Editing mode and return the input.
    Enter(String),

    /// Exit the Editing mode.
    Esc,
}

/// Cursor has a unique End position because it is going to be at the
/// end of the input buffer most of the time.
enum CursorPosition {
    Pos(usize),
    End,
}

enum InputMode {
    Editing,
    Normal,
}
