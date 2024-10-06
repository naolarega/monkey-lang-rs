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
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        use TokenType::*;

        match self {
            ILLEGAL => "ILLEGAL",
            EOF => "EOF",
            IDENT => "IDENT",
            INT => "INT",
            ASSIGN => "=",
            PLUS => "+",
            COMMA => ",",
            SEMICOLON => ";",
            LPAREN => "(",
            RPAREN => ")",
            LBRACE => "{",
            RBRACE => "}",
            FUNCTION => "FUNCTION",
            LET => "LET",
        }
        .to_string()
    }
}

pub const KEYWORDS: [(&str, TokenType); 2] = [("fn", TokenType::FUNCTION), ("let", TokenType::LET)];

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
