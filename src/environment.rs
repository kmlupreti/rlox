use crate::{error::LoxError, lox_value::LoxValue, token::Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Result<LoxValue, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(LoxError::RuntimeError {
                line: name.line,
                msg: format!("undeclared variable '{}' found", name.lexeme),
            }),
        }
    }
    pub fn redefine(&mut self, name: Token, value: LoxValue) -> Result<LoxValue, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, value.clone());
            Ok(value)
        } else {
            Err(LoxError::RuntimeError {
                line: name.line,
                msg: format!("unable to assign to undeclared variable '{}'", name.lexeme),
            })
        }
    }
}
