use crate::{
    callable::Callable, error::LoxError, function::Function, interpreter::Interpreter,
    lox_value::LoxValue,
};
use std::time::{SystemTime, UNIX_EPOCH};

struct BuiltInFunction<'a> {
    name: &'a str,
    params: Option<&'a [&'a str]>,
}

pub fn declare_builtin_functions(interpreter: &mut Interpreter) {
    let mut functions = vec![];
    functions.push(BuiltInFunction {
        name: "clock",
        params: None,
    });

    for function in functions {
        let params = match function.params {
            Some(params) => Vec::from(params),
            None => vec![],
        };
        interpreter.globals.borrow_mut().define(
            String::from(function.name),
            LoxValue::Callable(Callable::Func(Function {
                name: String::from(function.name),
                params: params.iter().map(|s| String::from(*s)).collect(),
                body: vec![],
                closure: None,
            })),
        );
    }
}

pub fn run_builtin_function(function: &Function, line: usize) -> Result<LoxValue, LoxError> {
    match function.name.as_str() {
        "clock" => Ok(LoxValue::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as f64,
        )),
        unknown_function => Err(LoxError::CallError {
            msg: format!("undefined built-in function '{}' called", unknown_function),
            line,
        }),
    }
}
