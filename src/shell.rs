use crate::executor::Executor;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::state::ShellState;
use std::io::{self, Write};

pub struct Shell {
    pub state: ShellState,
    pub resolver: Resolver,
    pub executor: Executor,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            state: ShellState::new(),
            resolver: Resolver::new(),
            executor: Executor::new(),
        }
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

            let mut parser = Parser::new(tokens);

            let ast = match parser.parse() {
                Ok(ast) => ast,

                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            println!("{:#?}", ast);
            let resolver = Resolver::new();

            let resolved = match resolver.resolve(ast) {
                Ok(command) => command,
                Err(err) => {
                    eprintln!("{err}");
                    continue;
                }
            };

            println!("{resolved:#?}");
            let executor = Executor::new();

            if let Err(err) = executor.execute(&mut self.state,resolved) {
                eprintln!("{err}");
            }
        }
    }
}
