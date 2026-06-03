#[derive(Debug, PartialEq)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
