use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

pub struct Messages<'a> {
    pub messages: Vec<&'a str>,
}

impl<'a> Messages<'a> {
    pub fn new(messages: Vec<&'a str>) -> Self {
        Self { messages }
    }
}

impl Widget for Messages<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();

        let widget = List::new(messages)
            .style(Style::default().fg(Color::LightGreen))
            .block(Block::bordered().title("Chat"));

        widget.render(area, buf);
    }
}

#[cfg(test)]
mod test {
    use ratatui::style::Modifier;

    use super::*;

    #[test]
    fn render() {
        let widget = Messages::new(vec!["Message 1", "Message 2", "Message 3"]);
        let area = Rect::new(0, 0, 30, 6);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let row = |y| -> String {
            (0..area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
        };

        assert_eq!(row(0), "┌Chat────────────────────────┐");
        assert_eq!(row(1), "│0: Message 1                │");
        assert_eq!(row(2), "│1: Message 2                │");
        assert_eq!(row(3), "│2: Message 3                │");
        assert_eq!(row(4), "│                            │");
        assert_eq!(row(5), "└────────────────────────────┘");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }
}
