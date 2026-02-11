use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

pub struct Users<'a> {
    pub users: Vec<&'a str>,
}

impl<'a> Users<'a> {
    pub fn new(users: Vec<&'a str>) -> Self {
        Self { users }
    }
}

impl Widget for Users<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let users: Vec<ListItem> = self
            .users
            .iter()
            .map(|u| {
                let content = Line::from(Span::raw(u.to_string()));
                ListItem::new(content)
            })
            .collect();

        let widget = List::new(users)
            .style(Style::default().fg(Color::LightGreen))
            .block(Block::bordered().title("Users"));

        widget.render(area, buf);
    }
}

#[cfg(test)]
mod test {
    use ratatui::style::Modifier;

    use super::*;

    #[test]
    fn render() {
        let widget = Users::new(vec!["Alice", "Bob", "Charlie"]);
        let area = Rect::new(0, 0, 30, 6);

        let mut buffer = Buffer::empty(area);
        widget.render(area, &mut buffer);

        let row = |y| -> String {
            (0..area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
        };

        assert_eq!(row(0), "┌Users───────────────────────┐");
        assert_eq!(row(1), "│Alice                       │");
        assert_eq!(row(2), "│Bob                         │");
        assert_eq!(row(3), "│Charlie                     │");
        assert_eq!(row(4), "│                            │");
        assert_eq!(row(5), "└────────────────────────────┘");

        let style = buffer[(4, 0)].style();
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(!style.add_modifier.contains(Modifier::BOLD));
    }
}
