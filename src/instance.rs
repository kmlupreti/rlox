use std::collections::HashMap;

use crate::{class::Class, lox_value::LoxValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Class,
    pub fields: HashMap<String, LoxValue>,
}
