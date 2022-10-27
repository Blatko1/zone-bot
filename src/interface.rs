use std::io;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, terminal::CompletedFrame, Frame, Terminal, layout::{Layout, Direction, Constraint}};

use crate::input::{self, InputHandler, Interruption};

pub struct Interface<B: Backend> {
    terminal: Terminal<B>,
    input: InputHandler,
    input_mode: InputMode,
    exit: bool,
}

impl<B: Backend> Interface<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            input: InputHandler::new(),
            input_mode: InputMode::Editing,
            exit: false,
        }
    }

    pub fn render_ui(&mut self) -> io::Result<CompletedFrame> {
        let ui = |frame: &mut Frame<B>| {
            let chunks = Layout::default().direction(Direction::Vertical).margin(2).constraints([Constraint::Length(3), Constraint::Length(2)].as_ref()).split(area)
        };

        self.terminal.draw(ui)
    }

    pub fn process_controls(&mut self, event: KeyEvent) {}

    pub fn process_editing(&mut self, event: KeyEvent) {
        let interruption = match self.input.process_input(event) {
            Some(intr) => intr,
            None => {
                println!("input: {}", self.input);
                return;
            }
        };

        if let Some(interruption) = self.input.process_input(event) {
            match interruption {
                Interruption::Enter(buf) => println!("You entered: {buf}"),
                Interruption::Esc => self.exit = true,
            }
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Editing,
    Control,
}
