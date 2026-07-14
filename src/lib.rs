use crate::error::LoxResult;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::{fs::File, path::Path};

pub mod builtin_functions;
pub mod callable;
pub mod class;
pub mod environment;
pub mod error;
pub mod expresssion;
pub mod function;
pub mod instance;
pub mod interpreter;
pub mod lox_value;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod token_type;

pub fn run_file<P>(path: P) -> LoxResult<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Interpreter::new()
        .interpret(Parser::new(scanner::Scanner::new(&buffer).scan_tokens()?.clone()).parse())
}
pub fn run_prompt() -> LoxResult<()> {
    let mut stdin = io::stdin().lock();
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        let mut scanner = scanner::Scanner::new(&line);
        if let Ok(tokens) = scanner.scan_tokens()
            && let Err(e) = interpreter.interpret(Parser::new(tokens.clone()).parse())
        {
            eprintln!("{e}");
        };
    }
    Ok(())
}
