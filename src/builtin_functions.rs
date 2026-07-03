use crate::{
    callable::Callable, environment::Environment, error::LoxError, function::Function,
    lox_value::LoxValue,
};
use std::time::{SystemTime, UNIX_EPOCH};

struct BuiltInFunction<'a> {
    name: &'a str,
    params: Option<&'a [&'a str]>,
}

pub fn declare_builtin_functions(globals: &mut Environment) {
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
        globals.define(
            String::from(function.name),
            LoxValue::Callable(Callable::Func(Function {
                name: String::from(function.name),
                is_user_defined: false,
                params: params.iter().map(|s| String::from(*s)).collect(),
                body: vec![],
                closure: None,
            })),
        );
    }
}

pub fn run_builtin_function(function: &Function) -> Result<LoxValue, LoxError> {
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
}
