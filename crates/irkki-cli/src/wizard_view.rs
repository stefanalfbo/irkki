use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    widgets::Block,
};

use crate::widget::{Prompt, SimpleHeader};

pub struct Model {
    pub prompt: String,
    pub input: String,
    pub character_index: usize,
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
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let header = SimpleHeader::new("Setup");
    frame.render_widget(header, vertical[1]);

    let prompt = Prompt::new(&model.prompt, &model.input);
    frame.render_widget(prompt, vertical[2]);

    let space = 2;
    #[allow(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        (model.prompt.len() + model.character_index + space) as u16,
        vertical[2].y,
    ));
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
    fn render_and_set_cursor() {
        let model = Model {
            prompt: "Question?".to_string(),
            input: "Answer".to_string(),
            character_index: 3,
        };
        let (buffer, cursor) = render(&model);
        let rows: Vec<String> = (0..15)
            .map(|y| (0..40).map(|x| buffer[(x, y)].symbol()).collect::<String>())
            .collect();
        let row = |y| -> &str { rows[y as usize].as_str() };

        assert_eq!(row(0), "┌──────────────────────────────────────┐");
        assert_eq!(row(14), "└──────────────────────────────────────┘");

        let setup_y = rows
            .iter()
            .position(|r| r.contains("Setup"))
            .expect("Setup header should be rendered") as u16;
        let setup_x = row(setup_y).find('S').expect("Setup S should be rendered") as u16;

        let prompt_y = rows
            .iter()
            .position(|r| r.contains("Question? Answer"))
            .expect("prompt should be rendered") as u16;
        let prompt_x = row(prompt_y)
            .find('Q')
            .expect("prompt Q should be rendered") as u16;

        let border_style = buffer[(0, 0)].style();
        assert_eq!(border_style.fg, Some(Color::LightGreen));

        let header_style = buffer[(setup_x, setup_y)].style();
        assert_eq!(header_style.fg, Some(Color::LightGreen));
        assert!(header_style.add_modifier.contains(Modifier::BOLD));

        let prompt_style = buffer[(prompt_x, prompt_y)].style();
        assert_eq!(prompt_style.fg, Some(Color::LightGreen));
        assert!(!prompt_style.add_modifier.contains(Modifier::BOLD));

        assert_eq!(
            cursor,
            Position::new(
                (model.prompt.len() + model.character_index + 2) as u16,
                prompt_y
            )
        );
    }
}
