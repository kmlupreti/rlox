use crate::{class::Class, function::Function, instance::Instance};
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Function(Function),
    Class(Class),
    Instance(Rc<RefCell<Instance>>),
    Null,
}
impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Null => write!(f, "nil"),
            LoxValue::Number(n) => write!(f, "{n}"),
            LoxValue::String(s) => write!(f, "\"{s}\""),
            LoxValue::Boolean(b) => write!(f, "{b}"),
            LoxValue::Class(class) => write!(f, "class: {}", class.name),
            LoxValue::Instance(instance) => {
                write!(f, "instance of class: {}", instance.borrow().class.name)
            }
            LoxValue::Function(function) => {
                write!(f, "fun {}()", function.name)
            }
        }
    }
}
impl LoxValue {
    pub fn is_true(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Null => false,
            _ => true,
        }
    }
}
