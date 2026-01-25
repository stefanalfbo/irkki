use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct Header<'a> {
    title: &'a str,
    subtitle: &'a str,
}

impl<'a> Header<'a> {
    pub fn new(title: &'a str, subtitle: &'a str) -> Self {
        Self { title, subtitle }
    }
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::LightGreen);
        let header = Paragraph::new(vec![
            Line::from(Span::styled(self.title, style.add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(self.subtitle)),
        ])
        .style(style)
        .alignment(Alignment::Center);

        header.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let widget = Header::new("Title", "Subtitle");
        let area = Rect::new(0, 0, 20, 5);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let header_line: String = (0..area.width).map(|x| buffer[(x, 0)].symbol()).collect();
        let subtitle_line: String = (0..area.width).map(|x| buffer[(x, 1)].symbol()).collect();

        assert_eq!(header_line, "        Title       ");
        assert_eq!(subtitle_line, "      Subtitle      ");

        let title_style = buffer[(8, 0)].style();
        assert_eq!(title_style.fg, Some(Color::LightGreen));
        assert!(title_style.add_modifier.contains(Modifier::BOLD));

        let subtitle_style = buffer[(7, 1)].style();
        assert_eq!(subtitle_style.fg, Some(Color::LightGreen));
        assert!(!subtitle_style.add_modifier.contains(Modifier::BOLD));
    }
}
