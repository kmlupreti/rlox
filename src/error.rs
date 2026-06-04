use crate::{token::Token, token_type::TokenType};

pub enum LoxError {
    UnexpectedChar { char: char, line: usize },
    ParseError { token: Token, msg: &'static str },
    UnterminatedString { line: usize },
    EvalError { msg: &'static str },
}
pub fn report_error(e: LoxError) {
    match e {
        LoxError::UnexpectedChar { char, line } => {
            eprintln!("[line: {}] Unexpected character {:?} found", line, char);
        }
        LoxError::UnterminatedString { line } => {
            eprintln!("[line: {}] Unterminated string", line);
        }
        LoxError::ParseError { token, msg } => {
            if token.token_type == TokenType::Eof {
                eprintln!("[line: {}] at end {}", token.line, msg);
            } else {
                eprintln!("[line: {}] at '{}' {}", token.line, token.lexeme, msg);
            }
        }
        LoxError::EvalError { msg } => {
            eprintln!("{}", msg);
        }
    }
}
