use crate::error::{LoxError, report_error};
use crate::token::{LiteralType, Token, TokenType};
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current_index: usize,
    has_error: bool,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self {
            source: source.chars().collect(),
            tokens,
            line: 1,
            start: 0,
            current_index: 0,
            has_error: false,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current_index >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current_index;
            if let Err(e) = self.scan_token() {
                self.has_error = true;
                report_error(e);
            }
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "\0".to_string(),
            self.line,
            LiteralType::Nil,
        ));
        if self.has_error {
            Err(())
        } else {
            Ok(&self.tokens)
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let lexeme = String::from_iter(&self.source[self.start..=self.current_index - 1]);
        self.tokens
            .push(Token::new(token_type, lexeme, self.line, literal));
    }
    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source[self.current_index - 1]
    }
    fn match_char(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if self.current_char() != expected_char {
                false
            } else {
                self.current_index += 1;
                true
            }
        }
    }
    fn current_char(&self) -> char {
        self.source[self.current_index]
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, LiteralType::Nil),

            ')' => self.add_token(TokenType::RightParen, LiteralType::Nil),

            '{' => self.add_token(TokenType::LeftBrace, LiteralType::Nil),

            '}' => self.add_token(TokenType::Rightbrace, LiteralType::Nil),

            '.' => self.add_token(TokenType::Dot, LiteralType::Nil),

            ';' => self.add_token(TokenType::Comma, LiteralType::Nil),

            ',' => self.add_token(TokenType::Semicolon, LiteralType::Nil),

            '+' => self.add_token(TokenType::Plus, LiteralType::Nil),

            '-' => self.add_token(TokenType::Minus, LiteralType::Nil),

            '*' => self.add_token(TokenType::Star, LiteralType::Nil),
            '\n' => self.line += 1,  // increment current line number
            ' ' | '\t' | '\r' => (), // skip whitespaces
            '/' => {
                if self.match_char('/') {
                    // skip single line comment
                    while self.current_char() != '\n' {
                        self.current_index += 1
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralType::Nil);
                }
            }
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    return Err(LoxError::UnterminatedString { line: self.line });
                }
                self.advance();
                let matched_string =
                    String::from_iter(&self.source[self.start + 1..self.current_index - 1]);
                self.add_token(TokenType::String, LiteralType::String(matched_string));
            }
            _ => {
                self.has_error = true;
                return Err(LoxError::UnexpectedChar {
                    char: c,
                    line: self.line,
                });
            }
        }
        Ok(())
    }
}
