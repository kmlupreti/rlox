use std::str;

use crate::{
    error::LoxError, expresssion::Expr, statement::Stmt, token::Token, token_type::TokenType,
};

type ParserResult<T> = Result<T, LoxError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }
    fn declaration(&mut self) -> Stmt {
        let stmt = if self.check(TokenType::Var) {
            self.var_declaration()
        } else if self.check(TokenType::Fun) {
            self.function("function")
        } else {
            self.statement()
        };
        match stmt {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                self.sync();
                Stmt::ExprStmt {
                    expr: Expr::Literal {
                        value: Token {
                            token_type: TokenType::Nil,
                            lexeme: String::new(),
                            line: self.peek().line,
                        },
                    },
                }
            }
        }
    }

    fn function(&mut self, kind: &str) -> ParserResult<Stmt> {
        self.advance();
        let name = self.consume(TokenType::Identifier, format!("expect {} name", kind))?;
        self.consume(
            TokenType::LeftParen,
            String::from("expect '(' after {kind} name"),
        )?;
        let mut params = vec![];
        if !self.check(TokenType::RightParen) {
            params = self.parameters()?;
        }
        self.consume(
            TokenType::RightParen,
            String::from("expect ')' after parameters"),
        )?;

        let statements = match self.statement()? {
            Stmt::BlockStmt { statements } => statements,
            other_statement => vec![other_statement],
        };
        Ok(Stmt::FuncStmt {
            name: name.lexeme,
            params,
            body: statements,
        })
    }
    fn parameters(&mut self) -> ParserResult<Vec<String>> {
        let mut params = vec![];
        params.push(
            self.consume(TokenType::Identifier, String::from("expected a parameter"))?
                .lexeme,
        );
        while self.check(TokenType::Comma) {
            self.advance();
            params.push(self.advance().lexeme.clone());
        }
        Ok(params)
    }
    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        self.advance();
        let name = self.consume(
            TokenType::Identifier,
            String::from("expected variable name"),
        )?;
        let mut initializer = Expr::Literal {
            value: Token {
                token_type: TokenType::Nil,
                lexeme: String::new(),
                line: self.peek().line,
            },
        };
        if self.check(TokenType::Equal) {
            self.advance();
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            String::from("expected ';' after value"),
        )?;
        Ok(Stmt::VarDeclStmt { name, initializer })
    }
    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.check(TokenType::Print) {
            self.print_stmt()
        } else if self.check(TokenType::LeftBrace) {
            self.block_stmt()
        } else if self.check(TokenType::If) {
            self.if_stmt()
        } else if self.check(TokenType::While) {
            self.while_stmt()
        } else if self.check(TokenType::For) {
            self.for_stmt()
        } else if self.check(TokenType::Return) {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn return_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            String::from("expected ';' after value"),
        )?;
        Ok(Stmt::ReturnStmt {
            keyword: self.peek().clone(),
            value,
        })
    }
    fn for_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        self.consume(TokenType::LeftParen, String::from("missing '('"))?;
        let initializer = if self.check(TokenType::Var) {
            Some(self.var_declaration()?)
        } else if self.check(TokenType::Semicolon) {
            self.advance();
            None
        } else {
            Some(self.expr_stmt()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal {
                value: Token {
                    token_type: TokenType::True,
                    lexeme: "true".to_string(),
                    line: self.peek().line,
                },
            }
        };
        self.consume(
            TokenType::Semicolon,
            String::from("expected ';'  after loop condition"),
        )?;
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, String::from("missing ')'"))?;
        let mut body = self.statement()?;
        if let Some(increment) = increment {
            if let Stmt::BlockStmt { mut statements } = body {
                let increment = Stmt::ExprStmt { expr: increment };
                statements.push(increment);
                body = Stmt::BlockStmt { statements };
            } else {
                body = Stmt::BlockStmt {
                    statements: vec![body, Stmt::ExprStmt { expr: increment }],
                };
            }
        }
        let while_stmt = Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        };
        if let Some(initializer) = initializer {
            Ok(Stmt::BlockStmt {
                statements: vec![initializer, while_stmt],
            })
        } else {
            Ok(while_stmt)
        }
    }
    fn while_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        self.consume(TokenType::LeftParen, String::from("missing '('"))?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, String::from("missing ')'"))?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::WhileStmt { condition, body })
    }
    fn if_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        self.consume(TokenType::LeftParen, String::from("missing '('"))?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, String::from("missing ')'"))?;
        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.check(TokenType::Else) {
            self.advance();
            else_branch = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }
    fn print_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance(); // consume String::from(print )token
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            String::from("expected ';' after value"),
        )?;
        Ok(Stmt::PrintStmt { expr })
    }
    fn expr_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            String::from("expected ';' after value"),
        )?;
        Ok(Stmt::ExprStmt { expr })
    }
    fn block_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        let mut statements = vec![];
        while !self.check(TokenType::Rightbrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(
            TokenType::Rightbrace,
            String::from("block is not closed as closing brace '}' is missing"),
        )?;
        Ok(Stmt::BlockStmt { statements })
    }
    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }
    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.logic_or()?;
        if self.check(TokenType::Equal) {
            self.advance(); // consume = 
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                })
            } else {
                Err(LoxError::RuntimeError {
                    line: self.peek().line,
                    msg: format!("unable to assign to invalid target "),
                })
            }
        } else {
            Ok(expr)
        }
    }
    fn logic_or(&mut self) -> ParserResult<Expr> {
        let left = self.logic_and()?;
        let operator = self.peek().clone();
        if self.check(TokenType::Or) {
            self.advance();
            let right = Some(Box::new(self.logic_and()?));
            Ok(Expr::Logical {
                left: Box::new(left),
                operator,
                right,
            })
        } else {
            Ok(left)
        }
    }
    fn logic_and(&mut self) -> ParserResult<Expr> {
        let left = self.equality()?;
        let operator = self.peek().clone();
        if self.check(TokenType::And) {
            self.advance();
            let right = Some(Box::new(self.equality()?));
            Ok(Expr::Logical {
                left: Box::new(left),
                operator,
                right,
            })
        } else {
            Ok(left)
        }
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
            self.call()
        }
    }
    fn call(&mut self) -> ParserResult<Expr> {
        let mut expr = self.primary()?;
        while self.check(TokenType::LeftParen) {
            let paren;
            let mut arguments = vec![];
            self.advance();
            if !self.check(TokenType::RightParen) {
                arguments = self.arguments()?;
                paren = self.consume(
                    TokenType::RightParen,
                    String::from("expected ')' after function arguments"),
                )?;
            } else {
                paren = self.peek().clone();
                self.advance();
            }
            expr = Expr::Call {
                callee: Box::new(expr),
                paren,
                arguments,
            };
        }
        Ok(expr)
    }
    fn arguments(&mut self) -> ParserResult<Vec<Expr>> {
        let mut arguments = vec![];
        arguments.push(self.expression()?);
        while self.check(TokenType::Comma) {
            if arguments.len() >= 255 {
                return Err(LoxError::RuntimeError {
                    line: self.peek().line,
                    msg: "can't have more than 255 arguments".to_string(),
                });
            }
            self.advance();
            arguments.push(self.expression()?);
        }
        Ok(arguments)
    }
    fn primary(&mut self) -> ParserResult<Expr> {
        match self.peek().token_type {
            TokenType::False | TokenType::True | TokenType::Nil => Ok(Expr::Literal {
                value: self.advance().clone(),
            }),
            TokenType::Number | TokenType::String => Ok(Expr::Literal {
                value: self.advance().clone(),
            }),
            TokenType::LeftParen => {
                self.advance();
                let expr = Box::new(self.expression()?);
                self.consume(
                    TokenType::RightParen,
                    String::from("expected ')' after expression"),
                )?;
                Ok(Expr::Grouping { expr })
            }
            TokenType::Identifier => Ok(Expr::Variable {
                name: self.advance().clone(),
            }),
            _ => Err(LoxError::ParseError {
                token: self.peek().clone(),
                msg: String::from("unexpected token found"),
            }),
        }
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
    fn consume(&mut self, token_type: TokenType, error_msg: String) -> ParserResult<Token> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(LoxError::ParseError {
                token: self.peek().clone(),
                msg: error_msg,
            })
        }
    }
    fn sync(&mut self) {
        while !self.is_at_end() {
            self.advance();
            if self.previous().token_type == TokenType::Semicolon {
                return;
            };
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::While
                | TokenType::For
                | TokenType::If
                | TokenType::Return
                | TokenType::Print => return,
                _ => continue,
            }
        }
    }
}
