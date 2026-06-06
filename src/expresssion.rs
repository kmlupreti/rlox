use std::fmt::Display;

use crate::error::LoxError;
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
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Literal { value } => {
                let value = value.lexeme.clone();
                write!(f, "{value}")
            }
            Self::Unary { operator, right } => {
                let operator = operator.lexeme.clone();
                write!(f, "({operator} {right})")
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let operator = operator.lexeme.clone();
                write!(f, "({operator} {left} {right})")
            }
            Self::Grouping { expr } => {
                write!(f, "(group {expr})")
            }
        }
    }
}

impl Expr {
    pub fn evaluate(&self) -> Result<LoxValue, LoxError> {
        match self {
            Self::Literal { value } => match value.token_type {
                TokenType::String => Ok(LoxValue::String(
                    (value.lexeme[1..value.lexeme.len() - 1]).to_string(),
                )),
                TokenType::Number => Ok(LoxValue::Number(value.lexeme.parse().unwrap())),
                TokenType::False => Ok(LoxValue::Boolean(false)),
                TokenType::True => Ok(LoxValue::Boolean(true)),
                TokenType::Nil => Ok(LoxValue::Null),
                _ => {
                    let lexeme = value.lexeme.clone();
                    Err(LoxError::RuntimeError {
                        line: value.line,
                        msg: format!("Illegal literal value '{lexeme}' found"),
                    })
                }
            },
            Self::Unary { operator, right } => {
                let lexeme = operator.lexeme.clone();
                let line = operator.line;
                let right = right.evaluate()?;
                match operator.token_type {
                    TokenType::Minus => {
                        let n = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line: operator.line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Number(-n))
                    }
                    TokenType::Bang => match right {
                        LoxValue::Boolean(b) => Ok(LoxValue::Boolean(!b)),
                        _ => Ok(LoxValue::Boolean(true)),
                    },
                    _ => Err(LoxError::RuntimeError {
                        line,
                        msg: format!("Illegal unary operator '{lexeme}' found"),
                    }),
                }
            }
            Self::Grouping { expr } => expr.evaluate(),
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                let token_type = operator.token_type;
                let lexeme = operator.lexeme.clone();
                let line = operator.line;
                match token_type {
                    TokenType::Minus => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Number(n1 - n2))
                    }
                    TokenType::Star => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Number(n1 * n2))
                    }
                    TokenType::Slash => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Number(n1 / n2))
                    }
                    TokenType::Plus => {
                        if let LoxValue::Number(n1) = left
                            && let LoxValue::Number(n2) = right
                        {
                            Ok(LoxValue::Number(n1 + n2))
                        } else if let LoxValue::String(s1) = left
                            && let LoxValue::String(s2) = right
                        {
                            Ok(LoxValue::String(format!("{}{}", s1, s2)))
                        } else {
                            Err(LoxError::RuntimeError {
                                line,
                                msg: format!(
                                    "failed to add/concat as operands should be both number or string"
                                ),
                            })
                        }
                    }
                    TokenType::Greater => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Boolean(n1 > n2))
                    }
                    TokenType::GreaterEqual => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Boolean(n1 >= n2))
                    }
                    TokenType::Less => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Boolean(n1 < n2))
                    }
                    TokenType::LessEqual => {
                        let n1 = match parse_num(&left) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{left}' into number"),
                                });
                            }
                        };
                        let n2 = match parse_num(&right) {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(LoxError::RuntimeError {
                                    line,
                                    msg: format!("failed to parse '{right}' into number"),
                                });
                            }
                        };
                        Ok(LoxValue::Boolean(n1 <= n2))
                    }
                    TokenType::EqualEqual => Ok(LoxValue::Boolean(left == right)),
                    TokenType::BangEqual => Ok(LoxValue::Boolean(left != right)),
                    _ => Err(LoxError::RuntimeError {
                        line,
                        msg: format!("Illegal binary operator '{lexeme}'"),
                    }),
                }
            }
        }
    }
}

fn parse_num(v: &LoxValue) -> Result<f64, ()> {
    match v {
        LoxValue::Number(n) => Ok(*n),
        LoxValue::String(s) => match s.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err(()),
        },
        _ => Err(()),
    }
}
