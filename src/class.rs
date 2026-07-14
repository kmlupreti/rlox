use crate::{
    callable::Callable,
    function::Function,
    instance::Instance,
    lox_value::{LoxValue, LoxValueResult},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
    pub super_class: Option<Box<Class>>,
}

impl Class {
    pub fn get_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }
}

impl Callable for Class {
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> LoxValueResult {
        self.check_args(args.len(), line)?;
        let instance = Rc::new(RefCell::new(Instance {
            class: self.clone(),
            fields: HashMap::new(),
        }));
        if let Some(initializer) = self.get_method("init") {
            let mut initializer = initializer.clone();
            initializer.bind(LoxValue::Instance(instance.clone()));
            initializer.call(interpreter, args, line)?;
        }
        Ok(LoxValue::Instance(instance))
    }
    fn arity(&self) -> usize {
        if let Some(initializer) = self.get_method("init") {
            initializer.params.len()
        } else {
            0
        }
    }
}
