use std::{io, marker::PhantomData};

use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
    terminal::CompletedFrame,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::input::{InputHandler, Interruption};

pub struct Interface<B: Backend> {
    terminal: Terminal<B>,
    ui: UI<B>,
    input: InputHandler,
    input_mode: InputMode,
    exit: bool,
}

impl<B: Backend> Interface<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            ui: UI::new(),
            input: InputHandler::new(),
            input_mode: InputMode::Editing,
            exit: false,
        }
    }

    pub fn render_ui(&mut self) -> io::Result<CompletedFrame> {
        let size = self.terminal.size().unwrap();

        self.terminal.draw(|f| self.ui.draw(f, size))
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

struct UI<B: Backend> {
    phantom: PhantomData<B>,
}

impl<B: Backend> UI<B> {
    fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    fn draw(&self, frame: &mut Frame<B>, size: Rect) {
        // Split the terminal into 2 parts left and right:
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(size);

        // ____________RENDER STUFF ONTO THE RIGHT PART OF THE TERMINAL: ____________
        self.draw_left_half(frame, chunks[0]);
        self.draw_right_half(frame, chunks[1]);
    }

    fn draw_left_half(&self, frame: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(7), Constraint::Min(6)])
            .split(area);
        
        self.draw_input_chunk(frame, chunks[0]);
        self.draw_else(frame, chunks[1]);
    }

    fn draw_right_half(&self, frame: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ])
            .split(area);

        let alert_items = vec![
            ListItem::new("mama ti!"),
            ListItem::new("tata ti!"),
            ListItem::new("baka ti!"),
        ];
        let alerts_list = List::new(alert_items)
            .start_corner(Corner::BottomLeft)
            .block(
                Block::default()
                    .title("Alerts List")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all())
                    .border_type(BorderType::Thick),
            );
        frame.render_widget(alerts_list, chunks[0]);

        let market_price = vec![Spans::from(vec![Span::from("Market price:")])];
        let price_paragraph = Paragraph::new(market_price).block(
            Block::default()
                .title("Live Price")
                .border_type(BorderType::Double)
                .borders(Borders::all()),
        );
        frame.render_widget(price_paragraph, chunks[1]);
    }

    fn draw_input_chunk(&self, frame: &mut Frame<B>, area: Rect) {
        let input_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(1), Constraint::Min(3)])
            .split(area);

        let block = Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Plain)
            .title("Add Zones");
        frame.render_widget(block, area);

        let 
    }

    // TODO
    fn draw_else(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().borders(Borders::all());
        frame.render_widget(block, area);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Editing,
    Control,
}
