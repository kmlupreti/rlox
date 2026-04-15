pub enum Error {
    UnexpectedChar { char: char, line: usize },
    ParseError,
}
pub fn report_error(e: Error) {
    match e {
        Error::UnexpectedChar { char, line } => {
            eprintln!("[line: {}] Unexpected character {:?} found", line, char);
        }
        _ => {
            eprintln!("Unknown error occured")
        }
    }
}
