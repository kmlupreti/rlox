use lox::error::LoxError;
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args().skip(1);
    if args.len() == 1 {
        if let Err(e) = lox::run_file(args.next().unwrap()) {
            let exit_code = if let LoxError::ScanError = e { 65 } else { 70 };
            eprintln!("{}", e);
            exit(exit_code);
        }
    } else if args.len() > 1 {
        eprintln!("usage: lox <script>");
        exit(64)
    } else {
        if let Err(e) = lox::run_prompt() {
            eprintln!("{}", e);
        }
    }
}
