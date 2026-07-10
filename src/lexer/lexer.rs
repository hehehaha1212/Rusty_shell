use super::scanner::Scanner;
use super::state::LexerState;
use super::token::Token;
// lexer state
pub struct Lexer<'a> {
    scanner: Scanner<'a>,
    state: LexerState,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
            state: LexerState::Normal,
            tokens: Vec::new(),
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        while !self.scanner.is_at_end() {
            match self.state {
                LexerState::Normal => {
                    self.lex_normal();
                }
            }
        }

        self.tokens.push(Token::EOF);

        self.tokens
    }

    fn lex_normal(&mut self) {
        let current = self.scanner.current().unwrap();

        match current {
            c if c.is_whitespace() => {
                self.scanner.advance();
            }

            '|' => {
                self.tokens.push(Token::Pipe);
                self.scanner.advance();
            }

            '<' => {
                self.tokens.push(Token::RedirectIn);
                self.scanner.advance();
            }

            '>' => {
                self.tokens.push(Token::RedirectOut);
                self.scanner.advance();
            }

            '&' => {
                self.tokens.push(Token::Background);
                self.scanner.advance();
            }

            ';' => {
                self.tokens.push(Token::Semicolon);
                self.scanner.advance();
            }

            '(' => {
                self.tokens.push(Token::LeftParen);
                self.scanner.advance();
            }

            ')' => {
                self.tokens.push(Token::RightParen);
                self.scanner.advance();
            }

            _ => {
                self.lex_word();
            }
        }
    }

    fn lex_word(&mut self) {
        let mut word = String::new();

        while let Some(c) = self.scanner.current() {
            if c.is_whitespace() {
                break;
            }

            match c {
                '|' | '<' | '>' | '&' | ';' | '(' | ')' => break,

                _ => {
                    word.push(c);
                    self.scanner.advance();
                }
            }
        }

        self.tokens.push(Token::Word(word));
    }
}
