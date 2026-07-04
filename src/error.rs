use crate::{lox_value::LoxValue, token::Token, token_type::TokenType};
use std::fmt::Display;

pub enum LoxError {
    ScanError,
    UnexpectedChar { char: char, line: usize },
    ParseError { token: Token, msg: String },
    UnterminatedString { line: usize },
    RuntimeError { line: usize, msg: String },
    CallError { msg: String, line: usize },
    Return { line: usize, value: Box<LoxValue> },
}
impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::UnexpectedChar { char, line } => {
                write!(f, "[line: {}] unexpected character {:?} found", line, char)
            }
            LoxError::UnterminatedString { line } => {
                write!(f, "[line: {}] unterminated string", line)
            }
            LoxError::ParseError { token, msg } => {
                if token.token_type == TokenType::Eof {
                    write!(f, "[line: {}] at end {}", token.line, msg)
                } else {
                    write!(f, "[line: {}] at '{}' {}", token.line, token.lexeme, msg)
                }
            }
            LoxError::RuntimeError { line, msg } => {
                write!(f, "[line: {}] {}", line, msg)
            }
            LoxError::CallError { msg, line } => {
                write!(f, "[line: {}] {}", line, msg)
            }
            LoxError::Return { line, value: _ } => {
                write!(
                    f,
                    "[line: {}] can't use return outside the function block",
                    line
                )
            }
            _ => write!(f, "unknown error occured"),
        }
    }
}
