use log::error;

use crate::{Lexer, Token, TokenType};

pub struct Message {
    pub prefix: Option<String>,
    pub command: String,
    pub params: Vec<String>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(message: &'a str) -> Self {
        let lexer = Lexer::new(message);

        Parser { lexer }
    }

    pub fn parse_message(&mut self) -> Message {
        let mut token = self.lexer.next_token();

        // Prefix handling
        let prefix = self.parse_prefix(&token);
        token = if let Some(_) = &prefix {
            // Move to the next token after prefix
            self.lexer.next_token()
        } else {
            token
        };

        let command = self.parse_command(&token);

        let params = self.parse_params();

        Message {
            prefix,
            command,
            params,
        }
    }

    fn parse_prefix(&mut self, token: &Token) -> Option<String> {
        if let TokenType::Colon = token.token_type {
            let prefix_token = self.lexer.next_token();
            if prefix_token.token_type != TokenType::Word {
                error!(
                    "parse_prefix: Expected prefix after ':', got {}",
                    prefix_token.literal
                );
                panic!("Expected prefix after ':'");
            }

            let space_token = self.lexer.next_token();
            if space_token.token_type != TokenType::Space {
                error!(
                    "parse_prefix: Expected space after prefix, got {}",
                    space_token.literal
                );
                panic!("Expected space after prefix");
            }

            return Some(prefix_token.literal.clone());
        }
        None
    }

    fn parse_command(&mut self, token: &Token) -> String {
        if token.token_type != TokenType::Word {
            error!(
                "parse_command: Expected command token, got {}",
                token.literal
            );
            panic!("Expected command token");
        } else if !Self::is_only_based_on_letters(&token.literal)
            && !Self::is_three_digit_number(&token.literal)
        {
            error!(
                "parse_command: Command must be letters or 3 digits, got {}",
                token.literal
            );
            panic!("Command must consist of letters only or a number with three digits.");
        } else {
            return token.literal.clone();
        }
    }

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        let mut token = self.lexer.next_token();

        if token.token_type == TokenType::CrLf || token.token_type == TokenType::EOF {
            return Vec::new();
        }

        while token.token_type == TokenType::Space {
            let mut param_token = self.lexer.next_token();

            // Skip extra spaces between params (e.g. broken servers).
            while param_token.token_type == TokenType::Space {
                param_token = self.lexer.next_token();
            }

            match param_token.token_type {
                TokenType::Word => {
                    if param_token.literal.starts_with(':') {
                        let mut trailing = param_token.literal[1..].to_string();

                        loop {
                            let next = self.lexer.next_token();
                            match next.token_type {
                                TokenType::CrLf | TokenType::EOF => break,
                                _ => trailing.push_str(&next.literal),
                            }
                        }

                        params.push(trailing);
                        return params;
                    } else {
                        params.push(param_token.literal);
                    }
                }
                TokenType::CrLf | TokenType::EOF => return params,
                _ => {
                    panic!("Expected parameter token")
                }
            }

            token = self.lexer.next_token();
            if token.token_type == TokenType::CrLf || token.token_type == TokenType::EOF {
                return params;
            }
        }

        if token.token_type != TokenType::CrLf && token.token_type != TokenType::EOF {
            error!(
                "parse_params: Expected new line or end of file, got {}",
                token.literal
            );
            panic!("Expected new line or end of file");
        }

        params
    }

    fn is_only_based_on_letters(value: &str) -> bool {
        value.chars().all(|c| c.is_ascii_alphabetic())
    }

    fn is_three_digit_number(value: &str) -> bool {
        if value.len() != 3 {
            return false;
        }

        value.chars().all(|c| c.is_ascii_digit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracting_prefix_from_message() {
        let message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("copper.libera.chat".to_string()),
            parsed_message.prefix
        );
    }

    #[test]
    fn test_message_without_prefix() {
        let message = "NOTICE * :*** Checking Ident\r\n";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
    }

    #[test]
    fn test_extracting_notice_command() {
        let message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!("NOTICE", parsed_message.command);
    }

    #[test]
    fn test_invalid_command_throws_exception() {
        let message = ":copper.libera.chat N0T1C3 * :*** Checking Ident\r\n";
        let mut parser = Parser::new(message);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parser.parse_message();
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_numeric_command() {
        let message = ":copper.libera.chat 001 copper :Welcome to the IRC server\r\n";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!("001", parsed_message.command);
    }

    #[test]
    fn parse_simple_message() {
        let message = "foo bar baz asdf";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), "asdf".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix() {
        let message = ":coolguy foo bar baz asdf";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), "asdf".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_trailing_param() {
        let message = "foo bar baz :asdf quux";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec![
                "bar".to_string(),
                "baz".to_string(),
                "asdf quux".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_empty_trailing_param() {
        let message = "foo bar baz :";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), "".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_colon_trailing_param() {
        let message = "foo bar baz ::asdf";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), ":asdf".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix_and_trailing() {
        let message = ":coolguy foo bar baz :asdf quux";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec![
                "bar".to_string(),
                "baz".to_string(),
                "asdf quux".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix_and_spacey_trailing() {
        let message = ":coolguy foo bar baz :  asdf quux ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec![
                "bar".to_string(),
                "baz".to_string(),
                "  asdf quux ".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_privmsg_trailing() {
        let message = ":coolguy PRIVMSG bar :lol :) ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("PRIVMSG", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "lol :) ".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix_and_empty_trailing() {
        let message = ":coolguy foo bar baz :";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), "".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix_and_blank_trailing() {
        let message = ":coolguy foo bar baz :  ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("coolguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string(), "  ".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_join_with_last_param() {
        let message = ":src JOIN #chan";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("src".to_string()), parsed_message.prefix);
        assert_eq!("JOIN", parsed_message.command);
        assert_eq!(vec!["#chan".to_string()], parsed_message.params);
    }

    #[test]
    fn parse_message_join_with_trailing_last_param() {
        let message = ":src JOIN :#chan";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("src".to_string()), parsed_message.prefix);
        assert_eq!("JOIN", parsed_message.command);
        assert_eq!(vec!["#chan".to_string()], parsed_message.params);
    }

    #[test]
    fn parse_message_away_without_param() {
        let message = ":src AWAY";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("src".to_string()), parsed_message.prefix);
        assert_eq!("AWAY", parsed_message.command);
        assert_eq!(Vec::<String>::new(), parsed_message.params);
    }

    #[test]
    fn parse_message_away_without_param_with_space() {
        let message = ":src AWAY ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("src".to_string()), parsed_message.prefix);
        assert_eq!("AWAY", parsed_message.command);
        assert_eq!(Vec::<String>::new(), parsed_message.params);
    }

    #[test]
    fn parse_message_tab_not_space() {
        let message = ":cool\tguy foo bar baz";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("cool\tguy".to_string()), parsed_message.prefix);
        assert_eq!("foo", parsed_message.command);
        assert_eq!(
            vec!["bar".to_string(), "baz".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_prefix_and_control_codes_1() {
        let message = ":coolguy!ag@net\x035w\x03ork.admin PRIVMSG foo :bar baz";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("coolguy!ag@net\x035w\x03ork.admin".to_string()),
            parsed_message.prefix
        );
        assert_eq!("PRIVMSG", parsed_message.command);
        assert_eq!(
            vec!["foo".to_string(), "bar baz".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_source_with_control_codes_2() {
        let message = ":coolguy!~ag@n\x02et\x0305w\x0fork.admin PRIVMSG foo :bar baz";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("coolguy!~ag@n\x02et\x0305w\x0fork.admin".to_string()),
            parsed_message.prefix
        );
        assert_eq!("PRIVMSG", parsed_message.command);
        assert_eq!(
            vec!["foo".to_string(), "bar baz".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_trailing_params() {
        let message = ":irc.example.com COMMAND param1 param2 :param3 param3";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("irc.example.com".to_string()), parsed_message.prefix);
        assert_eq!("COMMAND", parsed_message.command);
        assert_eq!(
            vec![
                "param1".to_string(),
                "param2".to_string(),
                "param3 param3".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_command_only() {
        let message = "COMMAND";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(None, parsed_message.prefix);
        assert_eq!("COMMAND", parsed_message.command);
        assert_eq!(Vec::<String>::new(), parsed_message.params);
    }

    #[test]
    fn parse_message_with_broken_unreal_erroneous_nick() {
        let message = ":gravel.mozilla.org 432  #momo :Erroneous Nickname: Illegal characters";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("gravel.mozilla.org".to_string()),
            parsed_message.prefix
        );
        assert_eq!("432", parsed_message.command);
        assert_eq!(
            vec![
                "#momo".to_string(),
                "Erroneous Nickname: Illegal characters".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_broken_unreal_mode_plus_n() {
        let message = ":gravel.mozilla.org MODE #tckk +n ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("gravel.mozilla.org".to_string()),
            parsed_message.prefix
        );
        assert_eq!("MODE", parsed_message.command);
        assert_eq!(
            vec!["#tckk".to_string(), "+n".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_with_broken_unreal_mode_plus_o() {
        let message = ":services.esper.net MODE #foo-bar +o foobar  ";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(
            Some("services.esper.net".to_string()),
            parsed_message.prefix
        );
        assert_eq!("MODE", parsed_message.command);
        assert_eq!(
            vec![
                "#foo-bar".to_string(),
                "+o".to_string(),
                "foobar".to_string()
            ],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_mode_trailing_plus_i() {
        let message = ":SomeOp MODE #channel :+i";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("SomeOp".to_string()), parsed_message.prefix);
        assert_eq!("MODE", parsed_message.command);
        assert_eq!(
            vec!["#channel".to_string(), "+i".to_string()],
            parsed_message.params
        );
    }

    #[test]
    fn parse_message_mode_trailing_user() {
        let message = ":SomeOp MODE #channel +oo SomeUser :AnotherUser";
        let mut parser = Parser::new(message);

        let parsed_message = parser.parse_message();

        assert_eq!(Some("SomeOp".to_string()), parsed_message.prefix);
        assert_eq!("MODE", parsed_message.command);
        assert_eq!(
            vec![
                "#channel".to_string(),
                "+oo".to_string(),
                "SomeUser".to_string(),
                "AnotherUser".to_string()
            ],
            parsed_message.params
        );
    }
}
