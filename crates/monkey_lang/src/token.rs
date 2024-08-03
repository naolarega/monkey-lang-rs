#[derive(PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT { literal: String },
    INT { literal: String },

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

impl ToString for Token {
    fn to_string(&self) -> String {
        use Token::*;

        match self {
            IDENT { literal } => format!("IDENT {{ {literal} }}"),
            INT { literal } => format!("INT {{ {literal} }}"),
            others => match others {
                ILLEGAL => "ILLEGAL",
                EOF => "",
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
                IDENT { literal: _ } | INT { literal: _ } => panic!("never reaches here"),
            }
            .to_string(),
        }
    }
}
