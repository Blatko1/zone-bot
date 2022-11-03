use std::io;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, terminal::CompletedFrame, Terminal};

use crate::{
    input::{InputHandler, Interruption},
    ui::UI,
};

pub struct Console<B: Backend> {
    terminal: Terminal<B>,
    ui: UI,

    input: InputHandler,
    input_mode: InputMode,

    exit: bool,
}

impl<B: Backend> Console<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            ui: UI::init(),

            input: InputHandler::new(),
            input_mode: InputMode::Editing,

            exit: false,
        }
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

    pub fn render_ui(&mut self) -> io::Result<CompletedFrame> {
        let size = self.terminal.size().unwrap();

        self.terminal.draw(|f| self.ui.render(f, size))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Editing,
    Control,
}