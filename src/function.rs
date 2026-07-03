use crate::{environment::Environment, statement::Stmt};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub is_user_defined: bool,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Option<Environment>,
}
