use crate::{error::LoxError, lox_value::LoxValue, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Environment {
    pub values: HashMap<String, LoxValue>,
    pub enclosing: Option<EnvRef>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_enclosing(enclosing: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Result<LoxValue, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => match self.enclosing {
                Some(ref env) => env.borrow().get(name),
                None => Err(LoxError::RuntimeError {
                    line: name.line,
                    msg: format!("undeclared identifier '{}' found", name.lexeme),
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
                Some(ref mut env) => env.borrow_mut().assign(name, value),
                None => Err(LoxError::RuntimeError {
                    line: name.line,
                    msg: format!(
                        "unable to assign to undeclared indentifier '{}'",
                        name.lexeme
                    ),
                }),
            }
        }
    }
}
