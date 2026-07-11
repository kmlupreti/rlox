use crate::{
    error::{LoxError, LoxResult},
    interpreter::Interpreter,
    lox_value::LoxValue,
};

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> Result<LoxValue, LoxError>;
    fn arity(&self) -> usize;
    fn check_args(&self, args_count: usize, line: usize) -> LoxResult<()> {
        if args_count >= 255 {
            return Err(LoxError::CallError {
                msg: String::from("can't have more than 255 arguments"),
                line,
            });
        } else if self.arity() != args_count {
            return Err(LoxError::CallError {
                msg: format!("expected {} arguments but got {}", self.arity(), args_count),
                line,
            });
        }
        Ok(())
    }
}
