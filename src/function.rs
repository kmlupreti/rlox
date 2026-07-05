use std::collections::HashMap;

use crate::{lox_value::LoxValue, statement::Stmt};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: HashMap<String, LoxValue>,
}
