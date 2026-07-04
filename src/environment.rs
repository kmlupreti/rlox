use crate::{error::LoxError, lox_value::LoxValue, token::Token};
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Result<LoxValue, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => match self.enclosing {
                Some(ref env) => env.get(name),
                None => Err(LoxError::RuntimeError {
                    line: name.line,
                    msg: format!("undeclared variable '{}' found", name.lexeme),
                }),
            },
        }
    }
    pub fn assign(&mut self, name: Token, value: LoxValue) -> Result<LoxValue, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, value.clone());
            Ok(value)
        } else {
            match self.enclosing {
                Some(ref mut env) => env.assign(name, value),
                None => Err(LoxError::RuntimeError {
                    line: name.line,
                    msg: format!("unable to assign to undeclared variable '{}'", name.lexeme),
                }),
            }
        }
    }
}
