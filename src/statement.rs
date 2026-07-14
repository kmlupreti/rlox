use crate::{expresssion::Expr, token::Token};

#[derive(Debug, Clone, PartialEq)]
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
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    FuncStmt {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    ClassStmt {
        name: Token,
        methods: Vec<Stmt>,
        super_class: Option<Expr>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
}
