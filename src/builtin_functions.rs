use crate::{
    error::LoxError,
    function::Function,
    interpreter::Interpreter,
    lox_value::{LoxValue, LoxValueResult},
};
use std::{
    collections::HashMap,
    io,
    time::{SystemTime, UNIX_EPOCH},
};

struct BuiltInFunction<'a> {
    name: &'a str,
    params: &'a [&'static str],
}

pub fn declare_builtin_functions(interpreter: &mut Interpreter) {
    let mut functions = vec![];
    functions.push(BuiltInFunction {
        name: "clock",
        params: &[],
    });
    functions.push(BuiltInFunction {
        name: "input",
        params: &["msg"],
    });

    for function in functions {
        interpreter.globals.borrow_mut().define(
            String::from(function.name),
            LoxValue::Function(Function {
                name: String::from(function.name),
                params: function.params.iter().map(|s| String::from(*s)).collect(),
                body: vec![],
                closure: None,
            }),
        );
    }
}

pub fn run_builtin_function(
    function: &Function,
    values: HashMap<String, LoxValue>,
    line: usize,
) -> LoxValueResult {
    match function.name.as_str() {
        "clock" => Ok(LoxValue::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as f64,
        )),
        "input" => {
            if let Some(msg) = values.get("msg") {
                println!("{}", msg);
                let stdin = io::stdin();
                let mut line = String::new();
                stdin.read_line(&mut line)?;
                Ok(LoxValue::String(line))
            } else {
                Err(LoxError::CallError {
                    msg: String::from("input message not found"),
                    line,
                })
            }
        }
        unknown_function => Err(LoxError::CallError {
            msg: format!("undefined built-in function '{}' called", unknown_function),
            line,
        }),
    }
}
