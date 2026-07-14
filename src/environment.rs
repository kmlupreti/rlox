use crate::{
    error::LoxError,
    lox_value::{LoxValue, LoxValueResult},
    token::Token,
};
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
    pub fn get_at(&self, name: &str, line: usize, distance: usize) -> LoxValueResult {
        if distance == 0
            && let Some(v) = self.values.get(name)
        {
            Ok(v.clone())
        } else if let Some(env) = self.ancestor(distance)
            && let Some(v) = env.borrow().values.get(name)
        {
            Ok(v.clone())
        } else {
            Err(LoxError::RuntimeError {
                line,
                msg: format!("undeclared identifier '{}' found", name),
            })
        }
    }
    pub fn ancestor(&self, distance: usize) -> Option<EnvRef> {
        let mut current_env = self.enclosing.as_ref()?.clone();
        for _ in 1..distance {
            let enclosing = current_env.borrow().enclosing.as_ref()?.clone();
            current_env = enclosing;
        }
        Some(current_env)
    }
    pub fn assign_at(&mut self, name: Token, value: LoxValue, distance: usize) -> LoxValueResult {
        if distance == 0 && self.values.contains_key(&name.lexeme) {
            Ok(self.values.insert(name.lexeme, value).unwrap())
        } else if let Some(env) = self.ancestor(distance)
            && env.borrow().values.contains_key(&name.lexeme)
        {
            env.borrow_mut().values.insert(name.lexeme, value.clone());
            Ok(value)
        } else {
            Err(LoxError::RuntimeError {
                line: name.line,
                msg: format!(
                    "Unable to assign to undeclared identifier '{}'",
                    name.lexeme
                ),
            })
        }
    }
}
