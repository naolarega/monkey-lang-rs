use std::env::{var, VarError};

use monkey_lang::repl::Repl;

fn main() {
    let repl = Repl::default();

    match get_user() {
        Ok(user) => {
            println!("Hello {user}! This is the monkey programming language!");
            println!("Feel free to type in commands");
        }
        Err(error) => panic!("{}", error),
    }

    repl.start();
}

fn get_user() -> Result<String, VarError> {
    if cfg!(windows) {
        var("USERNAME")
    } else {
        var("USER")
    }
}
