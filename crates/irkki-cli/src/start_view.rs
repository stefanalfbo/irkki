use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Block,
};

use crate::widget::{Button, Header};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartSelection {
    Start,
    Exit,
}

pub struct Model {
    pub selection: StartSelection,
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
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let header = Header::new("irkki", "An IRC client");
    frame.render_widget(header, vertical[1]);

    let options_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(12),
            Constraint::Length(4),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(vertical[3]);

    let start_button = Button::new("Start", model.selection == StartSelection::Start);
    frame.render_widget(start_button, options_row[1]);

    let exit_button = Button::new("Exit", model.selection == StartSelection::Exit);
    frame.render_widget(exit_button, options_row[3]);
}
