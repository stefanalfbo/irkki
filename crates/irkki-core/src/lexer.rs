#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    EOF,
    CrLf,
    Colon,
    Space,
    Word,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    current_char: Option<char>,
    current_position: usize,
    read_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            current_char: None,
            current_position: 0,
            read_position: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current_char = if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        };
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn read_string(&mut self) -> String {
        let start = self.current_position;
        let mut next_char = self.peek_char();

        while next_char != '\r' && next_char != ' ' && next_char != '\0' {
            self.read_char();
            next_char = self.peek_char();
        }

        self.input[start..=self.current_position].to_string()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        match self.current_char {
            Some(c) => match c {
                ':' => {
                    if self.current_position == 0 && self.read_position == 1 {
                        // Special case for leading colon in prefix
                        token = Token {
                            token_type: TokenType::Colon,
                            literal: ":".to_string(),
                        };
                    } else {
                        token = Token {
                            token_type: TokenType::Word,
                            literal: self.read_string(),
                        };
                    }
                }
                ' ' => {
                    token = Token {
                        token_type: TokenType::Space,
                        literal: c.to_string(),
                    };
                }
                '\r' => {
                    if self.peek_char() == '\n' {
                        self.read_char();
                        token = Token {
                            token_type: TokenType::CrLf,
                            literal: "\r\n".to_string(),
                        };
                    } else {
                        token = Token {
                            token_type: TokenType::Illegal,
                            literal: c.to_string(),
                        };
                    }
                }
                _ => {
                    token = Token {
                        token_type: TokenType::Word,
                        literal: self.read_string(),
                    };
                }
            },
            None => {
                token = Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            }
        }

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = ":prefix COMMAND arg1 arg2\r\n";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenType::Colon, ":"),
            (TokenType::Word, "prefix"),
            (TokenType::Space, " "),
            (TokenType::Word, "COMMAND"),
            (TokenType::Space, " "),
            (TokenType::Word, "arg1"),
            (TokenType::Space, " "),
            (TokenType::Word, "arg2"),
            (TokenType::CrLf, "\r\n"),
            (TokenType::EOF, ""),
        ];

        for (expected_type, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_type);
            assert_eq!(token.literal, expected_literal);
        }
    }
}
