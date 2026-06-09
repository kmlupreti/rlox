use crate::interpreter::Interpreter;
use crate::parser::Parser;
use std::io::{self, BufRead, BufReader, Write};
use std::{fs::File, path::Path, process::exit};

pub mod environment;
pub mod error;
pub mod expresssion;
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
    let mut interpreter = Interpreter::new();

    while reader.read_line(&mut buffer)? > 0 {
        let mut scanner = scanner::Scanner::new(&buffer);
        let tokens = match scanner.scan_tokens() {
            Ok(t) => t,
            Err(_) => exit(65),
        };

        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse();
        match interpreter.interpret(statements) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{e}");
                exit(70);
            }
        }
        buffer.clear();
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
        match interpreter.interpret(statements) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}
