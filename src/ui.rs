use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{alert::Alert, bot::MarketBot};

pub struct UI {
    // Static objects
    commands: CommandsPar,

    // Dynamic objects
    live_price: LivePricePar,
    alerts: AlertList,
    zone_list: ZoneList,
}

impl UI {
    pub fn init() -> Self {
        Self {
            commands: CommandsPar::new(),
            live_price: LivePricePar::new(),
            alerts: AlertList::new(),
            zone_list: ZoneList::new(),
        }
    }

    /// Updates the UI objects with
    pub fn update(&mut self, data: &MarketBot) {
        self.live_price.update(data);
        self.alerts.update(data);
        self.zone_list.update(data);
    }

    /// Updates layout and positions of the UI objects.
    pub fn update_layout(&mut self, terminal_area: Rect) {
        UILayout::top_bottom_layout(self, terminal_area);
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        self.commands.render(frame);
        self.live_price.render(frame);
        self.alerts.render(frame);
        self.zone_list.render(frame);
    }
}

#[derive(Debug)]
struct ZoneList {
    area: Rect,
    visible: bool,
}

impl ZoneList {
    fn new() -> Self {
        Self {
            area: Rect::default(),
            visible: true,
        }
    }
}

impl StaticObject for ZoneList {
    fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let block = Block::default().borders(Borders::all()).title("Zone List");
        frame.render_widget(block, self.area);
    }

    fn position_area(&mut self, terminal_size: Rect) {
        self.area = terminal_size;
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl DynamicObject for ZoneList {
    fn update(&mut self, bot: &MarketBot) {
        todo!()
    }
}

#[derive(Debug, Default)]
struct AlertList {
    area: Rect,
    visible: bool,
    alerts: Vec<Alert>,
}

impl AlertList {
    fn new() -> Self {
        Self {
            area: Rect::default(),
            visible: true,
            alerts: Vec::new(),
        }
    }
}

impl StaticObject for AlertList {
    fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let text = vec![];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::all()).title("Alerts"))
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, self.area);
    }

    fn position_area(&mut self, terminal_size: Rect) {
        self.area = terminal_size;
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl DynamicObject for AlertList {
    fn update(&mut self, data: &MarketBot) {
        todo!()
    }
}

#[derive(Debug)]
struct LivePricePar {
    area: Rect,
    visible: bool,

    symbol: String,
    price: String,
    // TODO volume:
    // 24h change:
    // 7d change:
}

impl LivePricePar {
    fn new() -> Self {
        Self {
            area: Rect::default(),
            visible: true,
            symbol: String::from("{Symbol}"),
            price: String::from("{Price}"),
        }
    }
}

impl StaticObject for LivePricePar {
    fn render<B: Backend>(&self, frame: &mut Frame<B>) {
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

        frame.render_widget(paragraph, self.area);
    }

    fn position_area(&mut self, terminal_size: Rect) {
        self.area = terminal_size;
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl DynamicObject for LivePricePar {
    fn update(&mut self, data: &MarketBot) {
        //self.symbol = data
        todo!()
    }
}

#[derive(Debug)]
struct CommandsPar {
    area: Rect,
    visible: bool,
}

impl CommandsPar {
    const HEIGHT: u16 = 6;

    fn new() -> Self {
        Self {
            area: Rect::default(),
            visible: true,
        }
    }
}

impl StaticObject for CommandsPar {
    fn render<B: Backend>(&self, frame: &mut Frame<B>) {
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

        frame.render_widget(paragraph, self.area);
    }

    fn position_area(&mut self, terminal_size: Rect) {
        self.area = terminal_size;
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

/// Every UI dynamic object which has constantly changing
/// data should implement [`DynamicObject`] trait.
trait DynamicObject: StaticObject {
    fn update(&mut self, data: &MarketBot);
}

// TODO
/// Every UI object should implement [`StaticObject`] trait
/// because it gives all the basic UI functions.
trait StaticObject {
    /// Every object has it's own position on the UI which is specified
    /// by this function. It takes the terminal dimensions as an argument
    /// and updated the object with the most suitable position area.
    /// Every time the terminal resizes this function should be called
    /// to update the object's position.
    fn position_area(&mut self, new_area: Rect);

    /// Renders the object to the provided [`Frame`] or in other words UI.
    fn render<B: Backend>(&self, frame: &mut Frame<B>);

    fn set_visibility(&mut self, visible: bool);

    fn is_visible(&self) -> bool;
}

struct UILayout;

impl UILayout {
    pub fn top_bottom_layout(ui: &mut UI, terminal_area: Rect) {
        // Splits the terminal into the top area and bottom object
        let top_area_bottom_object = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(terminal_area);

        ui.live_price.position_area(top_area_bottom_object[1]);

        {
            let left_right_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ])
                .split(top_area_bottom_object[0]);

            // Command help box, Zone list/input box
            {
                let left_objects = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(CommandsPar::HEIGHT),
                        Constraint::Min(4),
                    ])
                    .split(left_right_areas[0]);

                ui.commands.position_area(left_objects[0]);
                ui.zone_list.position_area(left_objects[1]);
            }

            // Alert list, Active Zones, Volume chart
            {
                let right_objects = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(10),
                        Constraint::Min(5),
                        Constraint::Length(6),
                    ])
                    .split(left_right_areas[1]);

                ui.alerts.position_area(right_objects[0]);
            }
        }
    }
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
