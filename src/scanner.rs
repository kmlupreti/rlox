use crate::error::{Error, report_error};
use crate::token::{LiteralType, Token, TokenType};
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    current_line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self {
            source,
            tokens,
            current_line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<(), ()> {
        self.current_line = 0;
        let mut errors: Vec<Error> = Vec::new();
        for c in self.source.clone().chars() {
            match c {
                '(' => {
                    self.add_token(TokenType::LeftParen, String::from(c), None);
                }
                ')' => {
                    self.add_token(TokenType::RightParen, String::from(c), None);
                }
                '{' => {
                    self.add_token(TokenType::LeftBrace, String::from(c), None);
                }
                '}' => {
                    self.add_token(TokenType::Rightbrace, String::from(c), None);
                }
                '.' => {
                    self.add_token(TokenType::Dot, String::from(c), None);
                }
                ',' => {
                    self.add_token(TokenType::Comma, String::from(c), None);
                }
                ';' => {
                    self.add_token(TokenType::Semicolon, String::from(c), None);
                }
                '+' => {
                    self.add_token(TokenType::Plus, String::from(c), None);
                }
                '-' => {
                    self.add_token(TokenType::Minus, String::from(c), None);
                }
                '*' => {
                    self.add_token(TokenType::Star, String::from(c), None);
                }
                ' ' | '\r' | '\t' => continue,
                '\n' => self.current_line += 1,
                _ => errors.push(Error::UnexpectedChar {
                    char: c,
                    line: self.current_line,
                }),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            for e in errors {
                report_error(e);
            }
            Err(())
        }
    }
    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<LiteralType>) {
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.current_line));
    }
}
