use crate::interpreter::Interpreter;
use crate::parser::Parser;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::{fs::File, path::Path, process::exit};

pub mod builtin_functions;
pub mod callable;
pub mod environment;
pub mod error;
pub mod expresssion;
pub mod function;
pub mod interpreter;
pub mod lox_value;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod token_type;

pub fn run_file<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let mut scanner = scanner::Scanner::new(&buffer);
    let tokens = match scanner.scan_tokens() {
        Ok(t) => t,
        Err(_) => exit(65),
    };
    let mut parser = Parser::new(tokens.clone());
    let statements = parser.parse();
    if let Err(e) = Interpreter::new().run(statements) {
        eprintln!("{e}");
        exit(70);
    }
    Ok(())
}
pub fn run_prompt() -> io::Result<()> {
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
        let tokens = match scanner.scan_tokens() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse();
        if let Err(e) = interpreter.run(statements) {
            eprintln!("{e}");
        }
    }
    Ok(())
}
