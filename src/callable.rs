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
                let mut values = interpreter.locals.values.clone();
                if let Some(env) = function.closure.clone() {
                    values.extend(env.values);
                }
                *interpreter.locals = Environment {
                    values,
                    enclosing: Some(interpreter.locals.clone()),
                };
                function.params.iter().enumerate().for_each(|(i, param)| {
                    interpreter.locals.define(param.clone(), args[i].clone());
                });
                let mut return_value = Ok(LoxValue::Null);
                if function.body.is_empty() {
                    return_value = run_builtin_function(function, line)
                } else {
                    for stmt in function.body.clone() {
                        match interpreter.interpret(stmt) {
                            Ok(_) => continue,
                            Err(LoxError::Return { line: _, value }) => {
                                return_value = Ok(*value);
                                break;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                interpreter.locals = interpreter.locals.enclosing.clone().unwrap();
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
