use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

pub struct Model {
    pub input: String,
    pub character_index: usize,
    pub messages: Vec<String>,
    pub users: Vec<String>,
}

pub fn view(model: &Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.area());
    let inner_layout =
        Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(outer_layout[0]);
    // let [messages_area, input_area] = inner_layout.areas(frame.area());

    let input = Paragraph::new(format!("> {}", model.input.as_str()))
        .style(Style::default().fg(Color::LightGreen))
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, inner_layout[1]);

    let space = 1;
    let next_row = 1; // under "Input" title border
    #[allow(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        ("> ".len() + model.character_index + space) as u16,
        (inner_layout[1].y + next_row) as u16,
    ));

    let messages: Vec<ListItem> = model
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages)
        .style(Style::default().fg(Color::LightGreen))
        .block(Block::bordered().title("Chat"));
    frame.render_widget(messages, inner_layout[0]);

    let users: Vec<ListItem> = model
        .users
        .iter()
        .enumerate()
        .map(|(_, u)| {
            let content = Line::from(Span::raw(format!("{u}")));
            ListItem::new(content)
        })
        .collect();

    let users = List::new(users)
        .style(Style::default().fg(Color::LightGreen))
        .block(Block::bordered().title("Users"));
    frame.render_widget(users, outer_layout[1]);
}
