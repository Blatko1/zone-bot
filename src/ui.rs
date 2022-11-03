use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::bot::Alert;

pub struct UI {
    commands: CommandsPar,
    live_price: LivePricePar,
    alerts: AlertList
}

impl UI {
    pub fn init() -> Self {
        Self {
            commands: CommandsPar::default(),
            live_price: LivePricePar::default(),
            alerts: AlertList::default()
        }
    }

    pub fn render<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        terminal_area: Rect,
    ) {
        let top_bottom = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(terminal_area);

        self.live_price.render(frame, top_bottom[1]);

        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(top_bottom[0]);

        let left_widgets = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(CommandsPar::HEIGHT),
                Constraint::Min(4),
            ])
            .split(left_right[0]);

        self.commands.render(frame, left_widgets[0]);

        let right_widgets = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(5),
                Constraint::Length(6),
            ])
            .split(left_right[1]);

        self.alerts.render(frame, right_widgets[0]);
    }
}

#[derive(Debug, Default)]
struct AlertList {
    alerts: Vec<Alert>
}

impl Renderable for AlertList {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let text = vec![];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::all()).title("Alerts")).alignment(Alignment::Left);
        frame.render_widget(paragraph, area);    
    }
}

#[derive(Debug)]
struct LivePricePar {
    symbol: String,
    price: String,
    // TODO volume:
    // 24h change:
    // 7d change:
}

impl LivePricePar {
    pub fn update(&mut self) {}
}

impl Renderable for LivePricePar {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let text = vec![Spans::from(vec![
            Span::styled(
                &self.symbol,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(": "),
            Span::raw(&self.price),
        ])];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::all()).title("Live Price"))
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}

impl Default for LivePricePar {
    fn default() -> Self {
        Self {
            symbol: String::from("{SYMBOL}"),
            price: String::from("{PRICE}"),
        }
    }
}

#[derive(Debug, Default)]
struct CommandsPar;

impl CommandsPar {
    const HEIGHT: u16 = 6;
}

impl Renderable for CommandsPar {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        // TODO maybe add custom owned struct instead of creating a new one
        // BOLD IS NOT VISIBLE
        let text = vec![
            Spans::from(vec![
                Span::styled(
                    "ESC",
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                Span::raw(" - exit the input mode"),
            ]),
            Spans::from(vec![
                Span::styled(
                    "ENTER",
                    Style::default().add_modifier(Modifier::SLOW_BLINK),
                ),
                Span::raw(" - confirm the input"),
            ]),
            Spans::from(vec![
                Span::styled(
                    "CTRL + N",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - add a new zone"),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::all()).title("Commands"))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

trait Renderable {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}

//// Draws the UI and holds all data used for drawing.
//struct UI<B: Backend> {
//    phantom: PhantomData<B>,
//}
//
//impl<B: Backend> UI<B> {
//    fn new() -> Self {
//        Self {
//            phantom: PhantomData,
//        }
//    }
//
//    fn draw(&self, frame: &mut Frame<B>, size: Rect) {
//        let chunks = Layout::default()
//            .direction(Direction::Vertical)
//            .margin(1)
//            .constraints([Constraint::Min(10), Constraint::Length(3)])
//            .split(size);
//
//        let price = vec![Spans::from(vec![Span::styled(
//            "{SYMBOL}",
//            Style::default().add_modifier(Modifier::BOLD),
//        ), Span::raw("{price}")])];
//        let live_price_widget = Paragraph::new(price)
//            .block(
//                Block::default()
//                    .title("Live Price")
//                    .borders(Borders::all())
//                    .style(Style::default().fg(Color::White).bg(Color::Black)),
//            )
//            .style(Style::default().fg(Color::White).bg(Color::Black))
//            .alignment(Alignment::Center);
//        frame.render_widget(live_price_widget, chunks[1]);
//    }
//
//    fn draw_left_half(&self, frame: &mut Frame<B>, area: Rect) {
//        // Split left half into 2 chunks:
//        let chunks = Layout::default()
//            .direction(Direction::Vertical)
//            .constraints([Constraint::Length(8), Constraint::Min(6)])
//            .split(area);
//
//        self.draw_zone_list(frame, chunks[0]);
//        self.draw_zone_chart(frame, chunks[1]);
//    }
//
//    fn draw_right_half(&self, frame: &mut Frame<B>, area: Rect) {
//        // Split the area into 3 chunks:
//        let chunks = Layout::default()
//            .direction(Direction::Vertical)
//            .constraints([
//                Constraint::Percentage(50),
//                Constraint::Length(4),
//                Constraint::Max(9),
//            ])
//            .split(area);
//
//        // Draw the first chunk:
//        let alert_items = [
//            ListItem::new("mama ti!"),
//            ListItem::new("tata ti!"),
//            ListItem::new("baka ti!"),
//        ];
//        let alerts_list = List::new(alert_items)
//            .start_corner(Corner::BottomLeft)
//            .block(
//                Block::default()
//                    .title("Alerts List")
//                    .title_alignment(Alignment::Center)
//                    .borders(Borders::all())
//                    .border_type(BorderType::Thick),
//            );
//        frame.render_widget(alerts_list, chunks[0]);
//
//        // Draw the second chunk:
//        let market_price = vec![Spans::from(vec![Span::from("Market price:")])];
//        let price_paragraph = Paragraph::new(market_price).block(
//            Block::default()
//                .title("Live Price")
//                .border_type(BorderType::Double)
//                .borders(Borders::all()),
//        );
//        frame.render_widget(price_paragraph, chunks[1]);
//
//        // Draw the third chunk:
//        let volume_chart = BarChart::default()
//            .block(
//                Block::default()
//                    .title("Volume chart")
//                    .borders(Borders::all()),
//            )
//            .bar_width(2)
//            .bar_gap(1)
//            .bar_set(bar::THREE_LEVELS)
//            .data(&[("B1", 1), ("B2", 4), ("B3", 3), ("B0", 6)]);
//        frame.render_widget(volume_chart, chunks[2]);
//    }
//
//    fn draw_zone_list(&self, frame: &mut Frame<B>, area: Rect) {
//        // Split area into 2 chunks:
//        let input_chunk = Layout::default()
//            .margin(1)
//            .direction(Direction::Horizontal)
//            .constraints([
//                Constraint::Percentage(40),
//                Constraint::Percentage(60),
//            ])
//            .split(area);
//
//        // Draw the surrounding barrier
//        let block = Block::default()
//            .borders(Borders::all())
//            .border_type(BorderType::Plain)
//            .title("Zone List");
//        frame.render_widget(block, area);
//
//        // Draw the zone list:
//        let list_items = [
//            ListItem::new("- Zone1"),
//            ListItem::new("- Zone2"),
//            ListItem::new("- Zone3"),
//            ListItem::new("- Zone4"),
//            ListItem::new("- Zone5"),
//        ];
//        let zone_list = List::new(list_items)
//            .block(Block::default().title("Zones").borders(Borders::all()))
//            .style(Style::default().fg(Color::White))
//            .highlight_symbol(">>")
//            .start_corner(Corner::TopLeft);
//        frame.render_widget(zone_list, input_chunk[0]);
//    }
//
//    // TODO
//    fn draw_zone_chart(&self, frame: &mut Frame<B>, area: Rect) {
//        let block = Block::default().borders(Borders::all());
//        frame.render_widget(block, area);
//    }
//}
