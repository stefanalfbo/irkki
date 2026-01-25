use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Widget},
};

pub struct ButtonWidget<'a> {
    label: &'a str,
    is_selected: bool,
}

impl<'a> ButtonWidget<'a> {
    pub fn new(label: &'a str, is_selected: bool) -> Self {
        Self { label, is_selected }
    }
}

impl Widget for ButtonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut style = Style::default().fg(Color::LightGreen);

        style = if self.is_selected {
            style.add_modifier(Modifier::BOLD)
        } else {
            style.fg(Color::Gray)
        };

        let button = Paragraph::new(self.label)
            .style(style)
            .alignment(Alignment::Center);

        button.render(area, buf);
    }
}
