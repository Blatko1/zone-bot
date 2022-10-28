use std::io;

use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    terminal::CompletedFrame,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

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
        let size = self.terminal.size().unwrap();
        let ui = |frame: &mut Frame<B>| {
            // Split the terminal into 2 parts left and right:
            let left_right_main = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [Constraint::Percentage(50), Constraint::Percentage(50)]
                        .as_ref(),
                )
                .split(size);
            // Split the right part into a big one on top and the small one on bottom:
            let top_bottom_second = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [Constraint::Percentage(90), Constraint::Percentage(10)]
                        .as_ref(),
                )
                .split(left_right_main[1]);

            // ____________RENDER STUFF ONTO THE RIGHT PART OF THE TERMINAL: ____________
            let items = vec![
                ListItem::new("mama ti!"),
                ListItem::new("tata ti!"),
                ListItem::new("baka ti!"),
            ];
            let alerts_list =
                List::new(items).start_corner(Corner::BottomLeft).block(
                    Block::default()
                        .title("Alerts List")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(alerts_list, top_bottom_second[0]);

            let text = vec![Spans::from(vec![Span::from("Market price:")])];
            let price_paragraph = Paragraph::new(text).block(
                Block::default()
                    .title("Live Price")
                    .border_type(BorderType::Double)
                    .borders(Borders::all()),
            );
            frame.render_widget(price_paragraph, top_bottom_second[1]);
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
