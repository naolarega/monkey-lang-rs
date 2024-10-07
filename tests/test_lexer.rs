use monkey_lang::{
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
