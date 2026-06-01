use crate::{error::LoxError, expresssion::Expr, token::Token, token_type::TokenType};

type ParserResult<T> = Result<T, LoxError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn match_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn consume(&mut self, token_type: TokenType, error_msg: &'static str) -> ParserResult<Token> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(LoxError::ParseError {
                token: self.peek().clone(),
                msg: error_msg,
            })
        }
    }
    fn expression(&mut self) -> ParserResult<Expr> {
        self.equality()
    }
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparision()?;
        while self.match_types(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparision()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }
        Ok(expr)
    }
    fn comparision(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;
        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }
        Ok(expr)
    }
    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;
        while self.match_types(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }
        Ok(expr)
    }
    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;
        while self.match_types(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }
        Ok(expr)
    }
    fn unary(&mut self) -> ParserResult<Expr> {
        if self.match_types(vec![TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> ParserResult<Expr> {
        match self.peek().token_type {
            TokenType::False | TokenType::True | TokenType::Nil => Ok(Expr::Literal {
                value: self.peek().clone(),
            }),
            TokenType::Number | TokenType::String => Ok(Expr::Literal {
                value: self.previous().clone(),
            }),
            TokenType::LeftParen => {
                let expr = Box::new(self.expression()?);
                let _ = self.consume(TokenType::RightParen, "Expected '(' after expression");
                Ok(Expr::Grouping { expr })
            }
            _ => Err(LoxError::ParseError {
                token: self.peek().clone(),
                msg: "error parsing primary expression",
            }),
        }
    }
}
