use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Widget},
};

pub struct Button<'a> {
    label: &'a str,
    is_selected: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str, is_selected: bool) -> Self {
        Self { label, is_selected }
    }
}

impl Widget for Button<'_> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn render_when_selected() {
        let widget = Button::new("Click Me", true);
        let area = Rect::new(0, 0, 12, 3);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let button_line: String = (0..area.width).map(|x| buffer[(x, 0)].symbol()).collect();
        assert_eq!(button_line, "  Click Me  ");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn render_when_not_selected() {
        let widget = Button::new("Click Me", false);
        let area = Rect::new(0, 0, 12, 3);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let button_line: String = (0..area.width).map(|x| buffer[(x, 0)].symbol()).collect();
        assert_eq!(button_line, "  Click Me  ");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::Gray));
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }
}
