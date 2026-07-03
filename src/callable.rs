use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    environment::Environment, error::LoxError, function::Function, interpreter::Interpreter,
    lox_value::LoxValue, statement::Stmt,
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
    ) -> Result<LoxValue, LoxError>;
    fn arity(&self) -> usize;
}
impl LoxCallable for Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, LoxError> {
        match self {
            Self::Func(function) => {
                let current_env = interpreter.environment.clone();
                let mut new_env = Environment::new_enclosing(
                    function
                        .closure
                        .clone()
                        .unwrap_or(interpreter.environment.clone()),
                );
                for i in 0..function.params.len() {
                    new_env.define(function.params[i].clone(), args[i].clone());
                }
                interpreter.environment = new_env;
                let return_value = if function.is_user_defined {
                    match interpreter.interpret(Stmt::BlockStmt {
                        statements: function.body.clone(),
                    }) {
                        Ok(()) => Ok(LoxValue::Null),
                        Err(LoxError::Return { line: _, value }) => Ok(value),
                        Err(e) => Err(e),
                    }
                } else {
                    match function.name.as_str() {
                        "clock" => Ok(LoxValue::Number(
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as f64,
                        )),
                        unknown_function => Err(LoxError::MiscError {
                            msg: format!("unknown built-in function '{}' called", unknown_function),
                        }),
                    }
                };
                interpreter.environment = current_env;
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
