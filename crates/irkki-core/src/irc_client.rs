use log::info;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::{Message, Parser};

#[derive(PartialEq)]
pub enum IRCEvent {
    Message(Message),
    Users(Vec<String>),
    MessageOfTheDay(Vec<String>),
    Raw(String),
}

impl std::fmt::Debug for IRCEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRCEvent::Message(msg) => write!(f, "IRCEvent::Message(cmd: {})", msg.command),
            IRCEvent::Raw(s) => write!(f, "IRCEvent::Raw({})", s),
            IRCEvent::Users(users) => write!(f, "IRCEvent::Users({})", users.join(", ")),
            IRCEvent::MessageOfTheDay(motd) => {
                write!(f, "IRCEvent::MessageOfTheDay({})", motd.join("\n"))
            }
        }
    }
}

pub struct IRCClient {
    nickname: String,
    server: String,
    port: u16,
    channel: String,
    reader: Option<BufReader<TcpStream>>,
    writer: Option<Arc<Mutex<BufWriter<TcpStream>>>>,
}

impl IRCClient {
    pub fn connect(
        nickname: impl Into<String>,
        server: impl Into<String>,
        port: u16,
    ) -> io::Result<Self> {
        let mut client = Self::new(nickname, server, port);
        client.connect_inner()?;
        Ok(client)
    }

    fn new(nickname: impl Into<String>, server: impl Into<String>, port: u16) -> Self {
        Self {
            nickname: nickname.into(),
            server: server.into(),
            port,
            channel: "#testchannel".to_string(),
            reader: None,
            writer: None,
        }
    }

    fn connect_inner(&mut self) -> io::Result<()> {
        let stream = TcpStream::connect((self.server.as_str(), self.port))?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = Arc::new(Mutex::new(BufWriter::new(stream)));

        self.reader = Some(reader);
        self.writer = Some(writer);

        self.send_line(&format!("NICK {}", self.nickname))?;
        self.send_line(&format!("USER {} 0 * :{}", self.nickname, self.nickname))?;
        self.send_line(&format!("JOIN {}", self.channel))?;

        Ok(())
    }

    pub fn send_message(&mut self, message: impl AsRef<str>) -> io::Result<()> {
        let message = message.as_ref().trim();
        if message.is_empty() {
            return Ok(());
        }

        self.send_line(&format!("PRIVMSG {} :{}", self.channel, message))
    }

    pub fn start_listening<F>(&mut self, mut message_handler: F) -> io::Result<JoinHandle<()>>
    where
        F: FnMut(IRCEvent) -> io::Result<()> + Send + 'static,
    {
        let mut reader = self.reader.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;
        let writer = self.writer.clone().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;

        Ok(thread::spawn(move || {
            let _ = Self::listen_loop(&mut reader, writer, &mut message_handler);
        }))
    }

    pub fn listen<F>(&mut self, mut message_handler: F) -> io::Result<()>
    where
        F: FnMut(IRCEvent) -> io::Result<()>,
    {
        let writer = self.writer.clone().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;
        let reader = self.reader.as_mut().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;
        Self::listen_loop(reader, writer, &mut message_handler)
    }

    fn listen_loop<F>(
        reader: &mut BufReader<TcpStream>,
        writer: Arc<Mutex<BufWriter<TcpStream>>>,
        message_handler: &mut F,
    ) -> io::Result<()>
    where
        F: FnMut(IRCEvent) -> io::Result<()>,
    {
        let mut message_of_the_day = Vec::new();
        loop {
            let mut line = String::new();
            let read_result = reader.read_line(&mut line);

            match read_result {
                Ok(0) => break,
                Ok(_) => {
                    info!("Received line: {}", line.trim_end());

                    let mut parser = Parser::new(&line);
                    let message = parser.parse_message();

                    match message.command.as_str() {
                        "PING" => {
                            let response = format!("PONG :{}", message.params.join(" "));
                            Self::send_line_with_writer(&writer, &response)?;
                            continue;
                        }
                        "353" => {
                            if let Some(names) = message.params.last() {
                                let mut users: Vec<String> = Vec::new();
                                for nick in names.split_whitespace() {
                                    if !nick.is_empty() {
                                        users.push(nick.to_string());
                                    }
                                }
                                message_handler(IRCEvent::Users(users))?;
                            }
                        }
                        "366" => {
                            info!("End of NAMES list.");
                        }
                        "375" => {
                            message_of_the_day.clear();
                        }
                        "372" => {
                            if let Some(motd_line) = message.params.last() {
                                message_of_the_day.push(motd_line.to_string());
                            }
                        }
                        "376" => {
                            message_handler(IRCEvent::MessageOfTheDay(message_of_the_day.clone()))?;
                        }
                        _ => {
                            message_handler(IRCEvent::Message(message))?;
                        }
                    }
                }
                Err(_) => break,
            }
        }

        Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        if self.writer.is_none() {
            return Ok(());
        }

        self.send_line(&format!("PART {} :Goodbye!", self.channel))?;
        self.send_line("QUIT :Client closed")?;

        Ok(())
    }

    fn send_line(&mut self, line: &str) -> io::Result<()> {
        let writer = self.writer.as_ref().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;
        Self::send_line_with_writer(writer, line)
    }

    fn send_line_with_writer(writer: &Arc<Mutex<BufWriter<TcpStream>>>, line: &str) -> io::Result<()> {
        let mut writer = writer.lock().map_err(|_| io::Error::other("Writer lock poisoned"))?;

        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\r\n")?;
        writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_without_connection_is_ok() {
        let mut client = IRCClient::new("nick", "localhost", 6667);

        assert!(client.quit().is_ok());
    }

    #[test]
    fn irc_event_debug_formats_message_variant() {
        let event = IRCEvent::Message(Message {
            prefix: None,
            command: "NOTICE".to_string(),
            params: vec!["#test".to_string(), "hello".to_string()],
        });

        assert_eq!(format!("{event:?}"), "IRCEvent::Message(cmd: NOTICE)");
    }

    #[test]
    fn irc_event_debug_formats_users_variant() {
        let event = IRCEvent::Users(vec!["alice".to_string(), "bob".to_string()]);

        assert_eq!(format!("{event:?}"), "IRCEvent::Users(alice, bob)");
    }

    #[test]
    fn irc_event_debug_formats_raw_variant() {
        let event = IRCEvent::Raw("Connected".to_string());

        assert_eq!(format!("{event:?}"), "IRCEvent::Raw(Connected)");
    }
}
