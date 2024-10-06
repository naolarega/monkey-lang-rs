use crate::{
    token::{look_up_ident, Token},
    utils::{is_digit, is_letter},
};

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

        self.skip_whitespace();

        let mut literal = None;
        let mut already_read = false;

        let token = Token::new(
            match self.ch {
                '=' if self.peek_char() == '=' => {
                    self.read_char();
                    literal = Some(String::from("=="));
                    EQ
                }
                '=' => ASSIGN,
                ';' => SEMICOLON,
                '(' => LPAREN,
                ')' => RPAREN,
                ',' => COMMA,
                '+' => PLUS,
                '-' => MINUS,
                '!' if self.peek_char() == '=' => {
                    self.read_char();
                    literal = Some(String::from("!="));
                    NOT_EQ
                }
                '!' => BANG,
                '/' => SLASH,
                '*' => ASTERISK,
                '<' => LT,
                '>' => GT,
                '{' => LBRACE,
                '}' => RBRACE,
                '\0' => EOF,
                _ => {
                    if is_letter(&self.ch) {
                        literal = Some(self.read_identifier());
                        already_read = true;

                        if let Some(ref ident) = literal {
                            if let Some(token_type) = look_up_ident(ident) {
                                token_type
                            } else {
                                IDENT
                            }
                        } else {
                            unreachable!()
                        }
                    } else if is_digit(&self.ch) {
                        literal = Some(self.read_number());
                        already_read = true;

                        INT
                    } else {
                        ILLEGAL
                    }
                }
            },
            if let Some(ident) = literal {
                ident
            } else {
                self.ch.to_string()
            },
        );

        if !already_read {
            self.read_char();
        }

        token
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return char::default();
        }

        if let Some(ch) = self
            .input
            .chars()
            .collect::<Vec<char>>()
            .get(self.read_position)
        {
            *ch
        } else {
            char::default()
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while is_digit(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while [' ', '\t', '\n', '\r'].contains(&self.ch) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
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
                Some(ch) => *ch,
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

    macro_rules! token_type_literal_pair {
        ($expected_type:ident, $expected_literal:expr) => {
            TokenTypeLiteralPair {
                expected_type: $expected_type,
                expected_literal: String::from($expected_literal),
            }
        };
    }

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            "#,
        );
        let tests = [
            token_type_literal_pair!(LET, "let"),
            token_type_literal_pair!(IDENT, "five"),
            token_type_literal_pair!(ASSIGN, "="),
            token_type_literal_pair!(INT, "5"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(LET, "let"),
            token_type_literal_pair!(IDENT, "ten"),
            token_type_literal_pair!(ASSIGN, "="),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(LET, "let"),
            token_type_literal_pair!(IDENT, "add"),
            token_type_literal_pair!(ASSIGN, "="),
            token_type_literal_pair!(FUNCTION, "fn"),
            token_type_literal_pair!(LPAREN, "("),
            token_type_literal_pair!(IDENT, "x"),
            token_type_literal_pair!(COMMA, ","),
            token_type_literal_pair!(IDENT, "y"),
            token_type_literal_pair!(RPAREN, ")"),
            token_type_literal_pair!(LBRACE, "{"),
            token_type_literal_pair!(IDENT, "x"),
            token_type_literal_pair!(PLUS, "+"),
            token_type_literal_pair!(IDENT, "y"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(RBRACE, "}"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(LET, "let"),
            token_type_literal_pair!(IDENT, "result"),
            token_type_literal_pair!(ASSIGN, "="),
            token_type_literal_pair!(IDENT, "add"),
            token_type_literal_pair!(LPAREN, "("),
            token_type_literal_pair!(IDENT, "five"),
            token_type_literal_pair!(COMMA, ","),
            token_type_literal_pair!(IDENT, "ten"),
            token_type_literal_pair!(RPAREN, ")"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(BANG, "!"),
            token_type_literal_pair!(MINUS, "-"),
            token_type_literal_pair!(SLASH, "/"),
            token_type_literal_pair!(ASTERISK, "*"),
            token_type_literal_pair!(INT, "5"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(INT, "5"),
            token_type_literal_pair!(LT, "<"),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(GT, ">"),
            token_type_literal_pair!(INT, "5"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(IF, "if"),
            token_type_literal_pair!(LPAREN, "("),
            token_type_literal_pair!(INT, "5"),
            token_type_literal_pair!(LT, "<"),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(RPAREN, ")"),
            token_type_literal_pair!(LBRACE, "{"),
            token_type_literal_pair!(RETURN, "return"),
            token_type_literal_pair!(TRUE, "true"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(RBRACE, "}"),
            token_type_literal_pair!(ELSE, "else"),
            token_type_literal_pair!(LBRACE, "{"),
            token_type_literal_pair!(RETURN, "return"),
            token_type_literal_pair!(FALSE, "false"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(RBRACE, "}"),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(EQ, "=="),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(INT, "10"),
            token_type_literal_pair!(NOT_EQ, "!="),
            token_type_literal_pair!(INT, "9"),
            token_type_literal_pair!(SEMICOLON, ";"),
            token_type_literal_pair!(EOF, "\0"),
        ];
        let mut lexer = Lexer::new(input);

        for expected_token in tests.iter() {
            let token = lexer.next_token();

            dbg!(token.literal(), &expected_token.expected_literal);

            assert!(token.token_type() == expected_token.expected_type);
            assert!(token.literal() == expected_token.expected_literal);
        }
    }
}
