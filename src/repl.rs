use std::io::{stdin, stdout, BufRead, BufReader, Read, Stdin, Stdout, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub struct Repl<R, W>
where
    R: Read,
    W: Write,
{
    reader: R,
    writer: W,
}

impl Default for Repl<Stdin, Stdout> {
    fn default() -> Self {
        Self {
            reader: stdin(),
            writer: stdout(),
        }
    }
}

impl<R, W> Repl<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub fn start(mut self) {
        let mut buf_reader = BufReader::new(self.reader);

        loop {
            self.writer.write(PROMPT.as_bytes()).unwrap();
            self.writer.flush().unwrap();

            let mut line = String::new();

            match buf_reader.read_line(&mut line) {
                Ok(_) => {
                    let lexer = Lexer::new(line);

                    for token in lexer {
                        write!(
                            self.writer,
                            "Token ( token_Type: {}, literal: {} )\n",
                            token.token_type(),
                            token.literal()
                        )
                        .unwrap();
                    }

                    self.writer.flush().unwrap();
                }
                Err(_) => (),
            }
        }
    }
}
