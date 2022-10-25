use crossterm::event::KeyEvent;
use tui::{backend::Backend, Terminal};

use crate::input::{Interruption, InputHandler};

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

    pub fn process_controls(&mut self, event: KeyEvent) {}

    pub fn process_editing(&mut self, event: KeyEvent) {
        self.terminal.clear().unwrap();
        let interruption = match self.input.process_input(event) {
            Some(intr) => intr,
            None => {
                println!("input: {}", self.input);
                return;
            }
        };

        match interruption {
            Interruption::Enter(buf) => println!("You entered: {buf}"),
            Interruption::Esc => self.exit = true,
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
