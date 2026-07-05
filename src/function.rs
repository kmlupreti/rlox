use crate::{environment::Environment, statement::Stmt};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Option<Rc<RefCell<Environment>>>,
}
