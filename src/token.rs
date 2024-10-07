use std::fmt::{Display, Formatter, Result};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // KeyWords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for TokenType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        use TokenType::*;

        let string_token_type = match self {
            ILLEGAL => "ILLEGAL",
            EOF => "EOF",
            IDENT => "IDENT",
            INT => "INT",
            ASSIGN => "=",
            PLUS => "+",
            MINUS => "-",
            BANG => "!",
            ASTERISK => "*",
            SLASH => "/",
            LT => "<",
            GT => ">",
            EQ => "==",
            NOT_EQ => "!=",
            COMMA => ",",
            SEMICOLON => ";",
            LPAREN => "(",
            RPAREN => ")",
            LBRACE => "{",
            RBRACE => "}",
            FUNCTION => "FUNCTION",
            LET => "LET",
            TRUE => "TRUE",
            FALSE => "FALSE",
            IF => "IF",
            ELSE => "ELSE",
            RETURN => "RETURN",
        };

        write!(formatter, "{}", string_token_type)
    }
}

pub const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", TokenType::FUNCTION),
    ("let", TokenType::LET),
    ("true", TokenType::TRUE),
    ("false", TokenType::FALSE),
    ("if", TokenType::IF),
    ("else", TokenType::ELSE),
    ("return", TokenType::RETURN),
];

pub fn look_up_ident(ident: &str) -> Option<TokenType> {
    KEYWORDS.iter().find_map(|(keyword, token_type)| {
        if *keyword == ident {
            Some(*token_type)
        } else {
            None
        }
    })
}

#[derive(PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}
