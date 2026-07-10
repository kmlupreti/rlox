use crate::{error::LoxError, interpreter::Interpreter, lox_value::LoxValue};

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> Result<LoxValue, LoxError>;
    fn arity(&self) -> usize;
}
