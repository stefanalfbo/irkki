use log::{debug, error, info};
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
        client.initialize_connection()?;
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

    fn initialize_connection(&mut self) -> io::Result<()> {
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

    /// The QUIT command is used to terminate a client’s connection to the server. The server
    /// acknowledges this by replying with an ERROR message and closing the connection to the
    /// client.
    fn quit(&mut self) -> io::Result<()> {
        if self.writer.is_none() {
            return Ok(());
        }

        self.send_line(&format!("PART {} :Goodbye!", self.channel))?;
        self.send_line("QUIT :Client closed")?;

        Ok(())
    }

    /// This command is used to query information about a particular user. The server SHOULD
    /// answer this command with numeric messages with information about the nick.
    ///
    /// The server SHOULD end its response (to a syntactically well-formed client message) with
    /// RPL_ENDOFWHOIS, even if it did not send any other numeric message. This allows clients to
    /// stop waiting for new numerics. In exceptional error conditions, servers MAY not reply to a
    /// WHOIS command. Clients SHOULD implement a hard timeout to avoid waiting for a reply which
    /// won’t come.
    ///
    /// Clients MUST NOT assume all numeric messages are sent at once, as servers can interleave
    /// other messages before the end of the WHOIS response.
    fn whois(&mut self, nickname: impl AsRef<str>) -> io::Result<()> {
        let nickname = nickname.as_ref().trim();
        if nickname.is_empty() {
            return Ok(());
        }

        self.send_line(&format!("WHOIS {}", nickname))
    }

    /// The NICK command is used to give the client a nickname or change the previous one.
    ///
    /// If the server receives a NICK command from a client where the desired nickname is
    /// already in use on the network, it should issue an ERR_NICKNAMEINUSE numeric and ignore
    /// the NICK command.
    ///
    /// If the server does not accept the new nickname supplied by the client as valid
    /// (for instance, due to containing invalid characters), it should issue an ERR_ERRONEUSNICKNAME
    /// numeric and ignore the NICK command. Servers MUST allow at least all alphanumerical
    /// characters, square and curly brackets ([]{}), backslashes (\), and pipe (|) characters in
    /// nicknames, and MAY disallow digits as the first character. Servers MAY allow extra
    /// characters, as long as they do not introduce ambiguity in other commands.
    fn change_nickname(&mut self, new_nickname: impl AsRef<str>) -> io::Result<()> {
        let new_nickname = new_nickname.as_ref().trim();
        if new_nickname.is_empty() {
            return Ok(());
        }

        self.send_line(&format!("NICK {}", new_nickname))?;
        self.nickname = new_nickname.to_string();
        Ok(())
    }

    pub fn send_message(&mut self, message: impl AsRef<str>) -> io::Result<()> {
        let message = message.as_ref().trim();
        if message.is_empty() {
            return Ok(());
        }

        if message.starts_with("/whois") {
            let nickname = message.trim_start_matches("/whois").trim();

            self.whois(&nickname)
        } else if message.starts_with("/nick") {
            let new_nick = message.trim_start_matches("/nick").trim();

            self.change_nickname(&new_nick)
        } else if message == "/quit" {
            self.quit()
        } else {
            self.send_line(&format!("PRIVMSG {} :{}", self.channel, message))
        }
    }

    pub fn start_listening<F>(&mut self, mut message_handler: F) -> io::Result<JoinHandle<()>>
    where
        F: FnMut(IRCEvent) -> io::Result<()> + Send + 'static,
    {
        let mut reader = self.reader.take().ok_or_else(|| {
            error!("Cannot start listening: Client is not connected.");
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;
        let writer = self.writer.clone().ok_or_else(|| {
            error!("Cannot start listening: Client is not connected.");
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;

        Ok(thread::spawn(move || {
            let _ = Self::listen_loop(&mut reader, writer, &mut message_handler);
        }))
    }

    fn listen_loop<F>(
        reader: &mut BufReader<TcpStream>,
        writer: Arc<Mutex<BufWriter<TcpStream>>>,
        message_handler: &mut F,
    ) -> io::Result<()>
    where
        F: FnMut(IRCEvent) -> io::Result<()>,
    {
        info!("Started listening for IRC messages.");

        let mut message_of_the_day = Vec::new();
        loop {
            let mut line = String::new();
            let read_result = reader.read_line(&mut line);

            match read_result {
                Ok(0) => {
                    info!("Connection closed by server.");
                    break;
                }
                Ok(_) => {
                    info!("Received line: {}", line.trim_end());

                    let mut parser = Parser::new(&line);
                    let message = parser.parse_message();

                    match message.command.as_str() {
                        "PING" => {
                            debug!("Received PING, sending PONG response.");
                            let response = format!("PONG :{}", message.params.join(" "));
                            Self::send_line_with_writer(&writer, &response)?;
                        }
                        // RPL_ENDOFWHOIS
                        "318" => {
                            debug!("Received end of WHOIS response.");
                        }
                        // RPL_NAMREPLY
                        "353" => {
                            if let Some(names) = message.params.last() {
                                let mut users: Vec<String> = Vec::new();
                                for nick in names.split_whitespace() {
                                    if !nick.is_empty() {
                                        users.push(nick.to_string());
                                    }
                                }
                                debug!("Received NAMES list: {}", users.join(", "));
                                message_handler(IRCEvent::Users(users))?;
                            }
                        }
                        // RPL_ENDOFNAMES
                        "366" => {
                            debug!("End of NAMES list.");
                        }
                        // RPL_MOTD
                        "372" => {
                            if let Some(motd_line) = message.params.last() {
                                message_of_the_day.push(motd_line.to_string());
                            }
                        }
                        // RPL_MOTDSTART
                        "375" => {
                            debug!("Start of MOTD.");
                            message_of_the_day.clear();
                        }
                        // RPL_ENDOFMOTD
                        "376" => {
                            debug!("End of MOTD.");
                            message_handler(IRCEvent::MessageOfTheDay(message_of_the_day.clone()))?;
                        }
                        // ERR_ERRONEUSNICKNAME
                        "432" => {
                            let nickname = message.params.get(1).cloned().unwrap_or_default();
                            error!("Nickname '{}' is invalid.", nickname);
                            message_handler(IRCEvent::Raw(format!(
                                "The nickname '{}' is invalid!",
                                nickname
                            )))?;
                        }
                        // ERR_NONICKNAMEGIVEN
                        "431" => {
                            error!("No nickname given.");
                            message_handler(IRCEvent::Raw(
                                "No nickname given! Please provide a nickname.".to_string(),
                            ))?;
                        }
                        // ERR_NICKNAMEINUSE
                        "433" => {
                            let nickname = message.params.get(1).cloned().unwrap_or_default();
                            error!("Nickname '{}' is already in use.", nickname);
                            message_handler(IRCEvent::Raw(format!(
                                "The nickname '{}' is already in use!",
                                nickname
                            )))?;
                        }
                        // ERR_NICKCOLLISION
                        "436" => {
                            let nickname = message.params.get(1).cloned().unwrap_or_default();
                            error!("Nickname '{}' is already in use (collision).", nickname);
                            message_handler(IRCEvent::Raw(format!(
                                "The nickname '{}' is already in use (collision)!",
                                nickname
                            )))?;
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

    fn send_line(&mut self, line: &str) -> io::Result<()> {
        let writer = self.writer.as_ref().ok_or_else(|| {
            error!("Cannot send line: Client is not connected.");
            io::Error::new(io::ErrorKind::NotConnected, "Client is not connected.")
        })?;

        debug!("Sending line: {}", line);
        Self::send_line_with_writer(writer, line)
    }

    fn send_line_with_writer(
        writer: &Arc<Mutex<BufWriter<TcpStream>>>,
        line: &str,
    ) -> io::Result<()> {
        let mut writer = writer
            .lock()
            .map_err(|_| io::Error::other("Writer lock poisoned"))?;

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
