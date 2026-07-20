#![allow(unused)]
use crate::{
    chunk::{Chunk, Value},
    error::{LoxError, LoxResult},
    expresssion::Expr,
    opcode::Opcode,
    statement::Stmt,
    token::Token,
    token_type::TokenType,
};

#[derive(Default)]
pub struct Compiler {
    statemets: Vec<Stmt>,
    chunk: Chunk,
    current_line: usize,
}
impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn compile(&mut self, statements: Vec<Stmt>) -> LoxResult<&Chunk> {
        for statement in statements {
            self.compile_stmt(statement)?;
        }
        self.emit_byte(Opcode::Return, self.current_line);
        Ok(&self.chunk)
    }
    fn emit_byte<T>(&mut self, byte: T, line: usize)
    where
        T: Into<u8>,
    {
        self.chunk.write_byte(byte, line);
    }
    fn emit_constant(&mut self, constant: Value) {
        let constant_index = self.chunk.add_constant(constant);
        self.emit_byte(Opcode::Constant, self.current_line);
        self.emit_byte(constant_index as u8, self.current_line);
    }
    fn emit_return(&mut self) {
        self.emit_byte(Opcode::Return, self.current_line);
    }
    pub fn compile_stmt(&mut self, statement: Stmt) -> LoxResult<()> {
        match statement {
            Stmt::Expr { expr } => self.compile_expression(expr)?,
            Stmt::Print { expr } => todo!(),
            Stmt::VarDecl { name, initializer } => todo!(),
            Stmt::Block { statements } => todo!(),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::While { condition, body } => todo!(),
            Stmt::Func { name, params, body } => todo!(),
            Stmt::Class {
                name,
                methods,
                super_class,
            } => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Break { keyword } => todo!(),
        }
        Ok(())
    }
    pub fn compile_expression(&mut self, expr: Expr) -> LoxResult<()> {
        match expr {
            Expr::Literal { value } => {
                self.current_line = value.line;
                let constant: Value = value.lexeme.parse().unwrap();
                self.emit_constant(constant);
            }
            Expr::Unary { operator, right } => todo!(),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;

                let lexeme = operator.lexeme.clone();
                let line = operator.line;
                match operator.token_type {
                    TokenType::Plus => {
                        self.emit_byte(Opcode::Add, line);
                    }
                    _ => {
                        return Err(LoxError::RuntimeError {
                            line,
                            msg: format!("uknown binary operator '{}' found", lexeme),
                        });
                    }
                }
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Grouping { expr } => todo!(),
            Expr::Identifier { name, id } => todo!(),
            Expr::Assign { name, value, id } => todo!(),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
            Expr::Get { name, expr } => todo!(),
            Expr::Set {
                name,
                value,
                object,
            } => todo!(),
            Expr::This { keyword, id } => todo!(),
            Expr::Super {
                keyword,
                method,
                id,
            } => todo!(),
        }
        Ok(())
    }
}
