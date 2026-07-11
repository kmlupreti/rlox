use crate::{
    callable::Callable, error::LoxError, function::Function, instance::Instance, token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
}
impl Callable for Class {
    fn call(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _args: Vec<crate::lox_value::LoxValue>,
        _line: usize,
    ) -> Result<crate::lox_value::LoxValue, crate::error::LoxError> {
        let instance = Rc::new(RefCell::new(Instance {
            class: self.clone(),
            fields: HashMap::new(),
        }));

        Ok(crate::lox_value::LoxValue::Instance(instance))
    }
    fn arity(&self) -> usize {
        0
    }
}
impl Class {
    pub fn get_method(&self, name: &Token) -> Result<Function, LoxError> {
        if let Some(method) = self.methods.get(&name.lexeme) {
            Ok(method.clone())
        } else {
            Err(LoxError::GetError {
                msg: format!("undefined method'{}'", name.lexeme),
                line: name.line,
            })
        }
    }
}
