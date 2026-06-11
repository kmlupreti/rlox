use crate::{expresssion::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    ExprStmt {
        expr: Expr,
    },
    PrintStmt {
        expr: Expr,
    },
    VarDeclStmt {
        name: Token,
        initializer: Expr,
    },
    BlockStmt {
        statements: Vec<Stmt>,
    },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}
