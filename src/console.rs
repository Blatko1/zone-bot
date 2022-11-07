use std::io;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, terminal::CompletedFrame, Terminal};

use crate::{
    bot::MarketBot,
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
        if let Some(interruption) = self.input.process_input(event) {
            match interruption {
                Interruption::Enter(buf) => println!("You entered: {buf}"),
                Interruption::Esc => self.exit = true,
            }
        }
    }

    pub fn update_ui(&mut self, data: &MarketBot) {
        self.ui.update(data);
    }

    pub fn render_ui(&mut self) -> io::Result<CompletedFrame> {
        self.terminal.draw(|f| self.ui.render(f))
    }

    pub fn resize(&mut self) {
        let size = self.terminal.size().unwrap();
        self.ui.update_layout(size);
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
