use crate::{
    builtin_functions::run_builtin_function, environment::Environment, error::LoxError,
    function::Function, interpreter::Interpreter, lox_value::LoxValue,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Func(Function),
    Class,
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> Result<LoxValue, LoxError>;
    fn arity(&self) -> usize;
}

impl LoxCallable for Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> Result<LoxValue, LoxError> {
        if args.len() >= 255 {
            return Err(LoxError::CallError {
                msg: String::from("can't have more than 255 arguments"),
                line,
            });
        }
        if self.arity() != args.len() {
            return Err(LoxError::CallError {
                msg: format!("expected {} arguments but got {}", self.arity(), args.len()),
                line,
            });
        }
        match self {
            Self::Func(function) => {
                let previous_env = interpreter.current_environment.clone();
                let function_env = Environment::new_enclosing(function.closure.clone().unwrap());
                function.params.iter().enumerate().for_each(|(i, param)| {
                    function_env
                        .borrow_mut()
                        .define(param.clone(), args[i].clone());
                });
                interpreter.current_environment = function_env;
                let mut return_value = Ok(LoxValue::Null);
                if function.body.is_empty() {
                    return_value = run_builtin_function(function, line)
                } else {
                    if let Err(LoxError::Return { line: _, value }) =
                        interpreter.run(function.body.clone())
                    {
                        return_value = Ok(*value);
                    }
                }
                interpreter.current_environment = previous_env;
                return_value
            }
            Self::Class => {
                todo!()
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Func(function) => function.params.len(),
            Self::Class => todo!(),
        }
    }
}
