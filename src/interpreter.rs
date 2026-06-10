use crate::{
    environment::Environment, error::LoxError, expresssion::Expr, lox_value::LoxValue,
    statement::Stmt, token_type::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            match statement {
                Stmt::ExprStmt { expr } => {
                    self.evaluate(expr)?;
                }
                Stmt::PrintStmt { expr } => {
                    let expr_out = self.evaluate(expr)?;
                    println!("{expr_out}");
                }
                Stmt::VarDeclStmt { name, initializer } => {
                    let value = self.evaluate(initializer)?;
                    self.environment.define(name.lexeme, value);
                }
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<LoxValue, LoxError> {
        match expr {
            Expr::Literal { value } => match value.token_type {
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
            Expr::Unary { operator, right } => {
                let lexeme = operator.lexeme.clone();
                let line = operator.line;
                let right = self.evaluate(*right)?;
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
            Expr::Grouping { expr } => self.evaluate(*expr),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
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
                                    "failed to add/concat as both operands should either number or string"
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
                        msg: format!("uknown binary operator '{lexeme}' found"),
                    }),
                }
            }
            Expr::Variable { name } => self.environment.get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.environment.redefine(name, value)
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
