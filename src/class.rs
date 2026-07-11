use crate::{
    callable::Callable,
    error::{LoxError, LoxResult},
    function::Function,
    instance::Instance,
    lox_value::{LoxValue, LoxValueResult},
    token::Token,
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
        _args: Vec<LoxValue>,
        _line: usize,
    ) -> LoxValueResult {
        let instance = Rc::new(RefCell::new(Instance {
            class: self.clone(),
            fields: HashMap::new(),
        }));
        Ok(LoxValue::Instance(instance))
    }
    fn arity(&self) -> usize {
        0
    }
}
impl Class {
    pub fn get_method(&self, name: &Token) -> LoxResult<Function> {
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
