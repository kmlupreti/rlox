use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Option<Box<Expr>>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Identifier {
        name: Token,
        id: usize,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
        id: usize,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}
