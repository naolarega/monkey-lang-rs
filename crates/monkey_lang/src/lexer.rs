use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new_lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn next_token(&mut self) -> Token {
        todo!();
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self
                .input
                .chars()
                .collect::<Vec<char>>()
                .get(self.read_position)
                .unwrap()
                .clone() as u8;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::{lexer::Lexer, token::Token};

    struct TokenTypeLiteralPair {
        expected_type: Token,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests = [
            TokenTypeLiteralPair {
                expected_type: Token::ASSIGN,
                expected_literal: String::from("="),
            },
            TokenTypeLiteralPair {
                expected_type: Token::PLUS,
                expected_literal: String::from("+"),
            },
            TokenTypeLiteralPair {
                expected_type: Token::LPAREN,
                expected_literal: String::from("("),
            },
            TokenTypeLiteralPair {
                expected_type: Token::RPAREN,
                expected_literal: String::from(")"),
            },
            TokenTypeLiteralPair {
                expected_type: Token::LBRACE,
                expected_literal: String::from("{"),
            },
            TokenTypeLiteralPair {
                expected_type: Token::RBRACE,
                expected_literal: String::from("}"),
            },
            TokenTypeLiteralPair {
                expected_type: Token::COMMA,
                expected_literal: String::from(","),
            },
            TokenTypeLiteralPair {
                expected_type: Token::SEMICOLON,
                expected_literal: String::from(";"),
            },
            TokenTypeLiteralPair {
                expected_type: Token::EOF,
                expected_literal: String::from(""),
            },
        ];
        let mut lexer = Lexer::new(input);

        for token_type in tests.iter() {
            let token = lexer.next_token();

            assert!(token == token_type.expected_type);
            assert!(token.to_string() == token_type.expected_literal);
        }
    }
}
