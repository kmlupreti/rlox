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
            let mut method = if let Some(class_method) = self.class.get_method(&name.lexeme) {
                class_method.clone()
            } else if let Some(super_class) = &self.class.super_class
                && let Some(super_class_method) = super_class.get_method(&name.lexeme)
            {
                super_class_method.clone()
            } else {
                return Err(LoxError::GetError {
                    msg: format!("undefined property or method '{}'", name.lexeme),
                    line: name.line,
                });
            };

            method.bind(instance);
            Ok(LoxValue::Function(method))
        }
    }
}
