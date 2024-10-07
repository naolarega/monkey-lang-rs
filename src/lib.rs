pub mod lexer;
pub mod repl;
pub mod token;

pub mod utils {
    pub fn is_letter(ch: &char) -> bool {
        ('a'..='z').contains(ch) || ('A'..='Z').contains(ch) || *ch == '_'
    }

    pub fn is_digit(ch: &char) -> bool {
        ('0'..='9').contains(ch)
    }
}
