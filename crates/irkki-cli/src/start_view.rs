use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

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

    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            "irkki",
            base_style.add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("A IRC client")),
    ])
    .style(base_style)
    .alignment(Alignment::Center);
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

    let start_style = if model.selection == StartSelection::Start {
        base_style.add_modifier(Modifier::BOLD)
    } else {
        base_style
    };
    let exit_style = if model.selection == StartSelection::Exit {
        base_style.add_modifier(Modifier::BOLD)
    } else {
        base_style
    };

    let start_block = Block::bordered().style(base_style).border_style(start_style);
    let start = Paragraph::new("Start")
        .style(start_style)
        .block(start_block)
        .alignment(Alignment::Center);
    frame.render_widget(start, options_row[1]);

    let exit_block = Block::bordered().style(base_style).border_style(exit_style);
    let exit = Paragraph::new("Exit")
        .style(exit_style)
        .block(exit_block)
        .alignment(Alignment::Center);
    frame.render_widget(exit, options_row[3]);
}
