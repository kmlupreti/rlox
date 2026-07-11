use crate::{
    class::Class,
    error::LoxError,
    lox_value::{LoxValue, LoxValueResult},
    token::Token,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Class,
    pub fields: HashMap<String, LoxValue>,
}
impl Instance {
    pub fn get(&self, name: &Token, instance: LoxValue) -> LoxValueResult {
        if let Some(v) = self.fields.get(&name.lexeme) {
            Ok(v.clone())
        } else {
            match self.class.get_method(&name.lexeme) {
                Some(method) => {
                    let mut method = method.clone();
                    method.bind(instance);
                    Ok(LoxValue::Function(method))
                }
                None => Err(LoxError::GetError {
                    msg: format!("undefined property or method '{}'", name.lexeme),
                    line: name.line,
                }),
            }
        }
    }
}
