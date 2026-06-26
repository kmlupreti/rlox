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
                let previous_environment = interpreter.globals.clone();
                let mut environment = Environment::new_enclosing(interpreter.globals.clone());
                for i in 0..function.params.len() {
                    environment.define(function.params[i].lexeme.clone(), args[i].clone());
                }
                interpreter.globals = environment;
                interpreter.interpret(Stmt::BlockStmt {
                    statements: function.body.clone(),
                })?;
                interpreter.globals = previous_environment;
                Ok(LoxValue::Null)
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
