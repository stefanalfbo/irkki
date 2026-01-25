use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::widget::SimpleHeader;

pub struct Model {
    pub prompt: String,
    pub input: String,
    pub character_index: usize,
}

pub fn view(model: &Model, frame: &mut Frame) {
    let base_style = Style::default().fg(Color::LightGreen);
    let area = frame.area();

    let outer_block = Block::bordered().style(base_style);
    frame.render_widget(&outer_block, area);
    let inner_area = outer_block.inner(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let header = SimpleHeader::new("Setup");
    frame.render_widget(header, vertical[1]);

    let prompt = Paragraph::new(vec![
        Line::from(Span::raw(&model.prompt)),
        Line::from(Span::raw(format!("> {}", model.input.as_str()))),
    ])
    .style(base_style)
    .block(Block::bordered())
    .alignment(Alignment::Left);
    frame.render_widget(prompt, vertical[2]);

    #[allow(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        vertical[2].x + model.character_index as u16 + 2,
        vertical[2].y + 2,
    ));
}
