use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new_lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn next_token(&mut self) -> Token {
        use crate::token::TokenType::*;

        let token = Token::new(
            match self.ch {
                '=' => ASSIGN,
                ';' => SEMICOLON,
                '(' => LPAREN,
                ')' => RPAREN,
                ',' => COMMA,
                '+' => PLUS,
                '{' => LBRACE,
                '}' => RBRACE,
                '\x00' => EOF,
                _ => panic!("unknown character"),
            },
            self.ch.to_string(),
        );

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = match self
                .input
                .chars()
                .collect::<Vec<char>>()
                .get(self.read_position)
            {
                Some(ch) => ch.clone(),
                None => char::default(),
            };
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::TokenType::{self, *},
    };

    struct TokenTypeLiteralPair {
        expected_type: TokenType,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests = [
            TokenTypeLiteralPair {
                expected_type: ASSIGN,
                expected_literal: String::from("="),
            },
            TokenTypeLiteralPair {
                expected_type: PLUS,
                expected_literal: String::from("+"),
            },
            TokenTypeLiteralPair {
                expected_type: LPAREN,
                expected_literal: String::from("("),
            },
            TokenTypeLiteralPair {
                expected_type: RPAREN,
                expected_literal: String::from(")"),
            },
            TokenTypeLiteralPair {
                expected_type: LBRACE,
                expected_literal: String::from("{"),
            },
            TokenTypeLiteralPair {
                expected_type: RBRACE,
                expected_literal: String::from("}"),
            },
            TokenTypeLiteralPair {
                expected_type: COMMA,
                expected_literal: String::from(","),
            },
            TokenTypeLiteralPair {
                expected_type: SEMICOLON,
                expected_literal: String::from(";"),
            },
            TokenTypeLiteralPair {
                expected_type: EOF,
                expected_literal: String::from("\x00"),
            },
        ];
        let mut lexer = Lexer::new(input);

        for token_type in tests.iter() {
            let token = lexer.next_token();

            assert!(token.token_type() == token_type.expected_type);
            assert!(token.literal() == token_type.expected_literal);
        }
    }
}
