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

        Message {
            prefix,
            command,
            params: vec![],
        }
    }

    fn parse_prefix(&mut self, token: &Token) -> Option<String> {
        if let TokenType::Colon = token.token_type {
            let prefix_token = self.lexer.next_token();
            if prefix_token.token_type != TokenType::Word {
                panic!("Expected prefix after ':'");
            }

            let space_token = self.lexer.next_token();
            if space_token.token_type != TokenType::Space {
                panic!("Expected space after prefix");
            }

            return Some(prefix_token.literal.clone());
        }
        None
    }

    fn parse_command(&mut self, token: &Token) -> String {
        if token.token_type != TokenType::Word {
            panic!("Expected command token");
        } else if !Self::is_only_based_on_letters(&token.literal)
            && !Self::is_three_digit_number(&token.literal)
        {
            panic!("Command must consist of letters only or a number with three digits.");
        } else {
            return token.literal.clone();
        }
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
}
