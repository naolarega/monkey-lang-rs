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
                EOF => "EOF",
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
                INT { literal: _ } | IDENT { literal: _ } => panic!("never reach here"),
            }
            .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
    }
}
