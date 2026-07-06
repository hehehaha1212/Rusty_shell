use crate::lexer::{Lexer, Token};
use std::io::{self, Write};
pub struct Shell {}

impl Shell {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&mut self) {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if (input.to_lowercase() == "exit") || (input.to_lowercase() == "quit") {
                break;
            }

            let lexer = Lexer::new(input);

            let tokens = lexer.lex();

            println!("{:#?}", tokens);
        }
    }
}
