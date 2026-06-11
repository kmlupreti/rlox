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

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        self.advance();
        let name = self.consume(TokenType::Identifier, "expected variable name")?;
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
        self.consume(TokenType::Semicolon, "expected ';' after value")?;
        Ok(Stmt::VarDeclStmt { name, initializer })
    }
    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.check(TokenType::Print) {
            self.print_stmt()
        } else if self.check(TokenType::LeftBrace) {
            self.block_stmt()
        } else if self.check(TokenType::If) {
            self.if_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn if_stmt(&mut self) -> ParserResult<Stmt> {
        self.advance();
        self.consume(TokenType::LeftParen, "missing '('")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "missing ')'")?;
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
        self.advance(); // consume print token
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expected ';' after value")?;
        Ok(Stmt::PrintStmt { expr })
    }
    fn expr_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expected ';' after value")?;
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
            "block is not closed as closing brace '}' is missing",
        )?;
        Ok(Stmt::BlockStmt { statements })
    }
    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }
    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.equality()?;
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
                value: self.advance().clone(),
            }),
            TokenType::Number | TokenType::String => Ok(Expr::Literal {
                value: self.advance().clone(),
            }),
            TokenType::LeftParen => {
                self.advance();
                let expr = Box::new(self.expression()?);
                self.consume(TokenType::RightParen, "expected ')' after expression")?;
                Ok(Expr::Grouping { expr })
            }
            TokenType::Identifier => Ok(Expr::Variable {
                name: self.advance().clone(),
            }),
            _ => Err(LoxError::ParseError {
                token: self.peek().clone(),
                msg: "unexpected token found",
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
