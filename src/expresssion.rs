use crate::{lox_value::LoxValue, token::Token, token_type::TokenType};

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
    pub fn print_ast(&self) -> String {
        match &self {
            Self::Literal { value } => value.lexeme.clone(),
            Self::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.print_ast())
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.print_ast(),
                    right.print_ast()
                )
            }
            Self::Grouping { expr } => {
                format!("(group {})", expr.print_ast())
            }
        }
    }
    pub fn evaluate(&self) -> LoxValue {
        match &self {
            Self::Literal { value } => match value.token_type {
                TokenType::String => {
                    LoxValue::String((value.lexeme[1..value.lexeme.len() - 1]).to_string())
                }
                TokenType::Number => LoxValue::Number(value.lexeme.parse().unwrap()),
                TokenType::False => LoxValue::Boolean(false),
                TokenType::True => LoxValue::Boolean(true),
                _ => LoxValue::Null,
            },
            Self::Unary { operator, right } => {
                let right = right.evaluate();
                match operator.token_type {
                    TokenType::Minus => match right {
                        LoxValue::Number(n) => LoxValue::Number(-n),
                        LoxValue::String(s) => LoxValue::Number(-s.parse::<f64>().unwrap()),
                        _ => LoxValue::Null,
                    },
                    TokenType::Bang => match right {
                        LoxValue::Boolean(b) => LoxValue::Boolean(!b),
                        _ => LoxValue::Boolean(true),
                    },
                    _ => LoxValue::Null,
                }
            }
            Self::Grouping { expr } => expr.evaluate(),
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate();
                let right = right.evaluate();
                match operator.token_type {
                    TokenType::Minus => {
                        if let LoxValue::Number(n1) = left
                            && let LoxValue::Number(n2) = right
                        {
                            LoxValue::Number(n1 - n2)
                        } else {
                            LoxValue::Null
                        }
                    }
                    TokenType::Star => {
                        if let LoxValue::Number(n1) = left
                            && let LoxValue::Number(n2) = right
                        {
                            LoxValue::Number(n1 * n2)
                        } else {
                            LoxValue::Null
                        }
                    }
                    TokenType::Plus => {
                        if let LoxValue::Number(n1) = left
                            && let LoxValue::Number(n2) = right
                        {
                            LoxValue::Number(n1 + n2)
                        } else if let LoxValue::String(s1) = left
                            && let LoxValue::Number(s2) = right
                        {
                            LoxValue::String(format!("{}{}", s1, s2))
                        } else {
                            LoxValue::Null
                        }
                    }
                    _ => LoxValue::Null,
                }
            }
        }
    }
}
