use crate::{
    builtin_functions::run_builtin_function,
    callable::Callable,
    environment::{EnvRef, Environment},
    error::LoxError,
    interpreter::Interpreter,
    lox_value::{LoxValue, LoxValueResult},
    statement::Stmt,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Option<EnvRef>,
}

impl Callable for Function {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
        line: usize,
    ) -> LoxValueResult {
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
        let previous_env = interpreter.current_environment.clone();
        let function_env =
            Environment::new_enclosing(self.closure.clone().unwrap_or(previous_env.clone()));
        self.params.iter().enumerate().for_each(|(i, param)| {
            function_env
                .borrow_mut()
                .define(param.clone(), args[i].clone());
        });
        interpreter.current_environment = function_env;
        let mut return_value = Ok(LoxValue::Null);
        if self.closure.is_none() {
            return_value = run_builtin_function(
                self,
                interpreter.current_environment.borrow().values.clone(),
                line,
            )
        } else {
            if let Err(LoxError::Return { line: _, value }) = interpreter.execute(self.body.clone())
            {
                return_value = Ok(*value);
            };
            if self.name == "init" {
                return_value = Ok(interpreter
                    .current_environment
                    .borrow()
                    .get_at("this", 1, 0)?);
            }
        }
        interpreter.current_environment = previous_env;
        return_value
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
impl Function {
    pub fn bind(&mut self, instance: LoxValue) {
        let instance_env = Environment::new_enclosing(self.closure.as_ref().unwrap().clone());
        instance_env
            .borrow_mut()
            .define("this".to_string(), instance);
        self.closure = Some(instance_env);
    }
}
