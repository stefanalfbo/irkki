use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub struct Prompt<'a> {
    pub message: &'a str,
    pub input: &'a str,
}

impl<'a> Prompt<'a> {
    pub fn new(message: &'a str, input: &'a str) -> Self {
        Self { message, input }
    }
}

impl Widget for Prompt<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::LightGreen);

        let prompt = Paragraph::new(vec![Line::from(Span::raw(format!(
            "{} {}",
            self.message, self.input
        )))])
        .style(style)
        .alignment(Alignment::Left);

        prompt.render(area, buf);
    }
}

#[cfg(test)]
mod test {
    use ratatui::style::Modifier;

    use super::*;

    #[test]
    fn render() {
        let widget = Prompt::new("Question?", "Answer");
        let area = Rect::new(0, 0, 30, 3);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let prompt: String = (0..area.width).map(|x| buffer[(x, 0)].symbol()).collect();
        assert_eq!(prompt, "Question? Answer              ");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }
}
