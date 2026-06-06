use crate::{error::LoxError, statement::Stmt};

pub struct Interpreter {
    statements: Vec<Stmt>,
}
impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
    pub fn interpret(&self) -> Result<(), LoxError> {
        for statement in &self.statements {
            statement.execute()?;
        }
        Ok(())
    }
}
