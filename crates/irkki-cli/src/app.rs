use color_eyre::Result;
use log::{debug, error};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};
use std::{
    io,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use crate::chat_view::{Model as ChatModel, view as chat_view};
use crate::start_view::{Model as StartModel, StartSelection, view as start_view};
use crate::wizard_view::{Model as WizardModel, view as wizard_view};
use irkki_core::{IRCClient, IRCEvent};

pub enum CurrentScreen {
    Start,
    Wizard,
    Chat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WizardStep {
    Nickname,
    Server,
    Port,
}
/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    messages: Vec<String>,
    users: Vec<String>,
    nickname: String,
    server: String,
    port: u16,
    current_screen: CurrentScreen,
    start_selection: StartSelection,
    wizard_step: WizardStep,
    incoming: Option<Receiver<IRCEvent>>,
    irc_client: Option<IRCClient>,
}

const INPUT_CHARACTER_START: usize = 3;

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            users: Vec::new(),
            character_index: 0,
            nickname: String::from("anonguest4523"),
            server: String::from("irc.eu.libera.chat"),
            port: 6667,
            current_screen: CurrentScreen::Start,
            start_selection: StartSelection::Start,
            wizard_step: WizardStep::Nickname,
            incoming: None,
            irc_client: None,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1) + INPUT_CHARACTER_START;
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1) + INPUT_CHARACTER_START;
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        let message = self.input.trim().to_string();

        debug!("Submitting message: {}", message);

        if !message.is_empty() {
            if let Some(client) = &mut self.irc_client {
                if let Err(error) = client.send_message(&message) {
                    self.messages
                        .push(format!("Failed to send message: {error}"));
                    error!("Failed to send message: {}", error);
                } else {
                    self.messages
                        .push(format!("<{}> {}", self.nickname, message));

                    if message == "/quit" {
                        self.current_screen = CurrentScreen::Start;
                    }
                }
            } else {
                self.messages
                    .push("Not connected to an IRC server.".to_string());
                error!("Failed to send message: Not connected to an IRC server.");
            }

            self.input.clear();
            self.reset_cursor();
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            self.drain_incoming();
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                let should_exit = match self.current_screen {
                    CurrentScreen::Start => self.handle_start_input(key.code),
                    CurrentScreen::Wizard => {
                        self.handle_wizard_input(key.code);
                        false
                    }
                    CurrentScreen::Chat => {
                        self.handle_chat_input(key.code);
                        false
                    }
                };

                if should_exit {
                    return Ok(());
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        match self.current_screen {
            CurrentScreen::Start => {
                let model = StartModel {
                    selection: self.start_selection,
                };
                start_view(&model, frame);
            }
            CurrentScreen::Wizard => {
                let model = WizardModel {
                    prompt: self.wizard_prompt(),
                    input: self.input.clone(),
                    character_index: self.character_index,
                };
                wizard_view(&model, frame);
            }
            CurrentScreen::Chat => {
                let model = ChatModel {
                    input: self.input.clone(),
                    character_index: self.character_index,
                    messages: self.messages.clone(),
                    users: self.users.clone(),
                };
                chat_view(&model, frame);
            }
        }
    }

    fn handle_start_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left | KeyCode::Right => {
                self.start_selection = match self.start_selection {
                    StartSelection::Start => StartSelection::Exit,
                    StartSelection::Exit => StartSelection::Start,
                };
                false
            }
            KeyCode::Enter => match self.start_selection {
                StartSelection::Start => {
                    self.current_screen = CurrentScreen::Wizard;
                    self.wizard_step = WizardStep::Nickname;
                    self.input.clear();
                    self.reset_cursor();
                    false
                }
                StartSelection::Exit => true,
            },
            _ => false,
        }
    }

    fn handle_wizard_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => self.advance_wizard(),
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            _ => {}
        }
    }

    fn handle_chat_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => self.submit_message(),
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            _ => {}
        }
    }

    fn wizard_prompt(&self) -> String {
        match self.wizard_step {
            WizardStep::Nickname => format!("Enter your nickname ({}):", self.nickname),
            WizardStep::Server => format!("Enter server address ({}):", self.server),
            WizardStep::Port => format!("Enter server port ({}):", self.port),
        }
    }

    fn advance_wizard(&mut self) {
        let trimmed = self.input.trim();
        match self.wizard_step {
            WizardStep::Nickname => {
                if !trimmed.is_empty() {
                    self.nickname = trimmed.to_string();
                }
                self.wizard_step = WizardStep::Server;
            }
            WizardStep::Server => {
                if !trimmed.is_empty() {
                    self.server = trimmed.to_string();
                }
                self.wizard_step = WizardStep::Port;
            }
            WizardStep::Port => {
                if !trimmed.is_empty() {
                    if let Ok(port) = trimmed.parse::<u16>() {
                        self.port = port;
                    } else {
                        return;
                    }
                }
                self.start_irc_connection();
                self.current_screen = CurrentScreen::Chat;
            }
        }

        self.input.clear();
        self.reset_cursor();
    }

    fn start_irc_connection(&mut self) {
        let server = self.server.clone();
        let port = self.port;
        let (sender, receiver) = mpsc::channel::<IRCEvent>();
        self.incoming = Some(receiver);

        let connect_result = IRCClient::connect(self.nickname.clone(), server.clone(), port);
        let mut client = match connect_result {
            Ok(client) => client,
            Err(error) => {
                self.messages
                    .push(format!("Failed to connect to {server}:{port}: {error}"));
                return;
            }
        };

        let listen_result = client.start_listening(move |event| {
            sender
                .send(event)
                .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e.to_string()))
        });
        if let Err(error) = listen_result {
            self.messages
                .push(format!("Failed to start listener: {error}"));
            return;
        }

        self.irc_client = Some(client);
    }

    fn drain_incoming(&mut self) {
        let receiver = match &self.incoming {
            Some(receiver) => receiver,
            None => return,
        };

        while let Ok(event) = receiver.try_recv() {
            match event {
                IRCEvent::Users(u) => {
                    self.users.extend(u);
                }
                IRCEvent::Message(message) => {
                    self.messages
                        .push(format!("{} {}", message.command, message.params.join(" ")));
                }
                IRCEvent::MessageOfTheDay(motd) => {
                    self.messages.push("Message of the Day:".to_string());
                    self.messages.extend(motd);
                }
                IRCEvent::Raw(raw) => self.messages.push(raw),
            }
        }
    }
}
