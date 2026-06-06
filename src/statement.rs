use crate::{error::LoxError, expresssion::Expr};

#[derive(Debug)]
pub enum Stmt {
    ExprStmt { expr: Expr },
    PrintStmt { expr: Expr },
}
impl Stmt {
    pub fn execute(&self) -> Result<(), LoxError> {
        match self {
            Self::ExprStmt { expr } => {
                expr.evaluate()?;
            }
            Self::PrintStmt { expr } => {
                let expr_out = expr.evaluate()?;
                println!("{expr_out}");
            }
        }
        Ok(())
    }
}
