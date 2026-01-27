use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
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
        let inner_height = area.height.saturating_sub(2) as usize;
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

        let total = self.messages.len();
        let scroll_offset = total.saturating_sub(inner_height);
        let mut list_state = ListState::default().with_offset(scroll_offset);
        StatefulWidget::render(widget, area, buf, &mut list_state);

        if inner_height > 0 && total > inner_height {
            let scrollbar_area = area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            });
            let mut scrollbar_state = ScrollbarState::new(total)
                .position(scroll_offset)
                .viewport_content_length(inner_height);
            StatefulWidget::render(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                scrollbar_area,
                buf,
                &mut scrollbar_state,
            );
        }
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

    #[test]
    fn render_with_scrollbar() {
        let messages: Vec<&str> = (1..=20)
            .map(|i| {
                let s: &'static mut str = Box::leak(format!("Message {}", i).into_boxed_str());
                &*s
            })
            .collect();
        let widget = Messages::new(messages);
        let area = Rect::new(0, 0, 30, 6);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let row = |y| -> String {
            (0..area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
        };

        assert_eq!(row(0), "┌Chat────────────────────────┐");
        assert_eq!(row(1), "│16: Message 17             ▲│");
        assert_eq!(row(2), "│17: Message 18             ║│");
        assert_eq!(row(3), "│18: Message 19             █│");
        assert_eq!(row(4), "│19: Message 20             ▼│");
        assert_eq!(row(5), "└────────────────────────────┘");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }
}
