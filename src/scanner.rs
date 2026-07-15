use crate::{error::LoxError, token::Token, token_type::TokenType};
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current_index: usize,
    has_error: bool,
}
impl Scanner {
    pub fn new(source: &str) -> Self {
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
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current_index;
            if let Err(e) = self.scan_token() {
                self.has_error = true;
                eprintln!("{e}");
            }
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "\0".to_string(), self.line));
        if self.has_error {
            Err(LoxError::ScanError)
        } else {
            Ok(&self.tokens)
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = String::from_iter(&self.source[self.start..=self.current_index - 1]);
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }
    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source[self.current_index - 1]
    }
    fn match_char(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            false
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
    fn peek_next(&self) -> char {
        if (self.current_index + 1) >= self.source.len() {
            '\0'
        } else {
            self.source[self.current_index + 1]
        }
    }
    fn match_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance(); // consume closing "
        }
        if self.is_at_end() {
            return Err(LoxError::UnterminatedString { line: self.line });
        }
        self.advance();
        self.add_token(TokenType::String);
        Ok(())
    }
    fn match_digits(&mut self) -> Result<(), LoxError> {
        while self.current_char().is_ascii_digit() {
            self.advance();
        }
        if self.current_char() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume decimal point
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
        Ok(())
    }
    fn match_keywords(&mut self) -> Result<(), LoxError> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = String::from_iter(&self.source[self.start..=self.current_index - 1]);
        let token_type = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "eof" => TokenType::Eof,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type);
        Ok(())
    }
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),

            ')' => self.add_token(TokenType::RightParen),

            '{' => self.add_token(TokenType::LeftBrace),

            '}' => self.add_token(TokenType::Rightbrace),

            '.' => self.add_token(TokenType::Dot),

            ';' => self.add_token(TokenType::Semicolon),

            ',' => self.add_token(TokenType::Comma),

            '+' => self.add_token(TokenType::Plus),

            '-' => self.add_token(TokenType::Minus),

            '*' => self.add_token(TokenType::Star),
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '\n' => self.line += 1,  // increment current line number
            ' ' | '\t' | '\r' => (), // skip whitespaces
            '/' => {
                if self.match_char('/') {
                    // skip single line comment
                    while !self.is_at_end() && self.current_char() != '\n' {
                        self.current_index += 1
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.match_string()?,
            '0'..='9' => self.match_digits()?,
            'a'..='z' | 'A'..='Z' | '_' => self.match_keywords()?,
            _ => {
                return Err(LoxError::UnexpectedChar {
                    char: c,
                    line: self.line,
                });
            }
        }
        Ok(())
    }
}
