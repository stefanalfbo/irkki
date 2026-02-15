use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::widget::{Messages, Users};

pub struct Model {
    pub input: String,
    pub character_index: usize,
    pub messages: Vec<String>,
    pub users: Vec<String>,
}

pub fn view(model: &Model, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.area());
    let inner_layout =
        Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(outer_layout[0]);

    let input = Paragraph::new(format!("> {}", model.input.as_str()))
        .style(Style::default().fg(Color::LightGreen))
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, inner_layout[1]);

    let space = 1;
    let next_row = 1; // under "Input" title border
    #[allow(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        ("> ".len() + model.character_index + space) as u16,
        (inner_layout[1].y + next_row) as u16,
    ));

    let messages = Messages::new(model.messages.iter().map(String::as_str).collect());
    frame.render_widget(messages, inner_layout[0]);

    let users = Users::new(model.users.iter().map(String::as_str).collect());
    frame.render_widget(users, outer_layout[1]);
}

#[cfg(test)]
mod test {
    use ratatui::{
        Terminal,
        backend::TestBackend,
        buffer::Buffer,
        layout::Position,
        style::{Color, Modifier},
    };

    use super::*;

    fn render(model: &Model) -> (Buffer, Position) {
        let backend = TestBackend::new(40, 15);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|frame| view(model, frame)).unwrap();
        let cursor = terminal.get_cursor_position().unwrap();
        (terminal.backend().buffer().clone(), cursor)
    }

    #[test]
    fn render_chat_widgets() {
        let model = Model {
            input: "hello".to_string(),
            character_index: 2,
            messages: vec!["Message 1".to_string(), "Message 2".to_string()],
            users: vec!["Alice".to_string(), "Bob".to_string()],
        };
        let (buffer, cursor) = render(&model);

        let rows: Vec<String> = (0..15)
            .map(|y| (0..40).map(|x| buffer[(x, y)].symbol()).collect::<String>())
            .collect();

        assert!(rows.iter().any(|r| r.contains("Chat")));
        assert!(rows.iter().any(|r| r.contains("Input")));
        assert!(rows.iter().any(|r| r.contains("> hello")));
        assert!(rows.iter().any(|r| r.contains("Users")));
        assert!(rows.iter().any(|r| r.contains("Alice")));
        assert!(rows.iter().any(|r| r.contains("0: Message 1")));

        let input_y = rows
            .iter()
            .position(|r| r.contains("> hello"))
            .expect("input line should be rendered") as u16;
        let input_x = rows[input_y as usize]
            .find('>')
            .expect("> should be rendered") as u16;
        let input_style = buffer[(input_x, input_y)].style();
        assert_eq!(input_style.fg, Some(Color::LightGreen));
        assert!(!input_style.add_modifier.contains(Modifier::BOLD));

        assert_eq!(cursor, Position::new(5, input_y));
    }

    #[test]
    fn render_sets_cursor_from_character_index() {
        let model = Model {
            input: "abcdef".to_string(),
            character_index: 4,
            messages: vec![],
            users: vec![],
        };
        let (_buffer, cursor) = render(&model);

        assert_eq!(cursor, Position::new(7, 13));
    }
}
