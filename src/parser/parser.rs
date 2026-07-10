use crate::lexer::Token;
use crate::parser::{AST, SimpleCommand};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    //only supports simple command currently
    pub fn parse(&mut self) -> Result<AST, String> {
        self.parse_simple_command()
    }

    fn advance(&mut self) {
        self.position += 1;
    }
    fn parse_simple_command(&mut self) -> Result<AST, String> {
        let mut words = Vec::new();

        while let Some(token) = self.current() {
            match token {
                Token::Word(word) => {
                    words.push(word.clone());
                    self.advance();
                }
                Token::EOF => break,
                _ => return Err("Unexpected Token".to_string()),
            }
        }
        if words.is_empty() {
            return Err("Empty Command".to_string());
        }
        let command = words.remove(0);
        Ok(AST::Simple(SimpleCommand {
            command,

            args: words,
        }))
    }
}
