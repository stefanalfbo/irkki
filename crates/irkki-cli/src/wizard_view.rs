use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    widgets::Block,
};

use crate::widget::{Prompt, SimpleHeader};

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

    let prompt = Prompt::new(&model.prompt, &model.input);
    frame.render_widget(prompt, vertical[2]);

    let space = 2;
    #[allow(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        (model.prompt.len() + model.character_index + space) as u16,
        vertical[2].y,
    ));
}
