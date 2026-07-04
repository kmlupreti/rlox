use std::env::args;
use std::io;
use std::process::exit;

fn main() -> io::Result<()> {
    let mut args = args().skip(1);
    if args.len() == 1 {
        lox::run_file(args.next().unwrap())?
    } else if args.len() > 1 {
        eprintln!("usage: lox <script>");
        exit(64)
    } else {
        lox::run_prompt()?
    }
    Ok(())
}
