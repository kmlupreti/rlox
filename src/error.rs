use crate::{lox_value::LoxValue, token::Token, token_type::TokenType};
use std::{fmt::Display, io::Error};

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Debug)]
pub enum LoxError {
    ScanError,
    UnexpectedChar { char: char, line: usize },
    ParseError { token: Token, msg: String },
    UnterminatedString { line: usize },
    RuntimeError { line: usize, msg: String },
    CallError { msg: String, line: usize },
    Return { line: usize, value: Box<LoxValue> },
    Break { line: usize },
    ResolveError { line: usize, msg: String },
    GetError { msg: String, line: usize },
    IOError { msg: String },
}
impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::UnexpectedChar { char, line } => {
                write!(
                    f,
                    "[line: {}] scan error:  unexpected character {:?} found",
                    line, char
                )
            }
            LoxError::UnterminatedString { line } => {
                write!(f, "[line: {}] scan error:  unterminated string", line)
            }
            LoxError::ParseError { token, msg } => {
                if token.token_type == TokenType::Eof {
                    write!(f, "[line: {}] parse error: at end {}", token.line, msg)
                } else {
                    write!(
                        f,
                        "[line: {}] at '{}' parse error: {}",
                        token.line, token.lexeme, msg
                    )
                }
            }
            LoxError::RuntimeError { line, msg } => {
                write!(f, "[line: {}] runtime error: {}", line, msg)
            }
            LoxError::CallError { msg, line } => {
                write!(f, "[line: {}] call error: {}", line, msg)
            }
            LoxError::GetError { msg, line } => {
                write!(f, "[line: {}] get error: {}", line, msg)
            }
            LoxError::ResolveError { msg, line } => {
                write!(f, "[line: {}] resolve error: {}", line, msg)
            }
            LoxError::Return { line, value: _ } => {
                write!(
                    f,
                    "[line: {}] error: can't use return outside the function block",
                    line
                )
            }
            LoxError::Break { line } => {
                write!(f, "[line: {}] error: can't use break outside loop", line)
            }
            LoxError::IOError { msg } => {
                write!(f, "I/O error: {}", msg)
            }
            _ => write!(f, "error: unknown error occured"),
        }
    }
}
impl From<Error> for LoxError {
    fn from(value: Error) -> Self {
        Self::IOError {
            msg: value.to_string(),
        }
    }
}
