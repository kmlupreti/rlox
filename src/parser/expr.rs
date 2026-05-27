use crate::scanner::token::Token;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn accept(&self) -> String {
        match &self {
            Self::Literal { value } => value.lexeme(),
            Self::Unary { operator, right } => {
                format!("( {} {} )", operator.lexeme(), right.accept())
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "( {} {} {})",
                    left.accept(),
                    operator.lexeme(),
                    right.accept()
                )
            }
            Self::Grouping { expr } => {
                format!("( {} )", expr.accept())
            }
        }
    }
}
