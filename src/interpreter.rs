use crate::{
    builtin_functions::declare_builtin_functions,
    callable::{Callable, LoxCallable},
    environment::Environment,
    error::LoxError,
    expresssion::Expr,
    function::Function,
    lox_value::LoxValue,
    statement::Stmt,
    token_type::TokenType,
};
#[derive(Default)]
pub struct Interpreter {
    pub globals: Box<Environment>,
    pub locals: Box<Environment>,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self::default();
        declare_builtin_functions(&mut interpreter.globals);
        interpreter
    }
    pub fn interpret(&mut self, statement: Stmt) -> Result<(), LoxError> {
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
                self.locals.define(name.lexeme, value);
            }
            Stmt::BlockStmt { statements } => {
                *self.locals = Environment::new_enclosing(self.locals.clone());
                let mut result = Ok(());
                for s in statements {
                    result = self.interpret(s);
                    if result.is_err() {
                        break;
                    };
                }
                self.locals = self.locals.enclosing.clone().unwrap();
                result?
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.evaluate(condition)?;
                if let LoxValue::Boolean(is_condition_true) = condition {
                    if is_condition_true {
                        self.interpret(*then_branch)?
                    } else {
                        if let Some(else_branch) = else_branch {
                            self.interpret(*else_branch)?
                        }
                    }
                }
            }

            Stmt::WhileStmt { condition, body } => {
                while self.evaluate(condition.clone())?.is_true() {
                    self.interpret(*body.clone())?;
                }
            }
            Stmt::FuncStmt { name, params, body } => {
                self.locals.define(
                    name.clone(),
                    LoxValue::Callable(Callable::Func(Function {
                        name,
                        params,
                        body,
                        closure: self.locals.values.clone(),
                    })),
                );
            }
            Stmt::ReturnStmt { keyword, value } => {
                let value = match value {
                    Some(expr) => self.evaluate(expr)?,
                    None => LoxValue::Null,
                };
                return Err(LoxError::Return {
                    line: keyword.line,
                    value: Box::new(value),
                });
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
                    TokenType::Bang => Ok(LoxValue::Boolean(!right.is_true())),
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
                                msg: String::from(
                                    "failed to add/concat as both operands should either number or string",
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
            Expr::Variable { name } => match self.globals.get(name.clone()) {
                Ok(v) => Ok(v),
                Err(_) => self.locals.get(name),
            },
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.locals.assign(name, value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                match operator.token_type {
                    TokenType::Or => {
                        if left.is_true() {
                            Ok(LoxValue::Boolean(true))
                        } else {
                            if let Some(right) = right {
                                Ok(LoxValue::Boolean(self.evaluate(*right)?.is_true()))
                            } else {
                                Ok(LoxValue::Boolean(false))
                            }
                        }
                    }
                    TokenType::And => {
                        if !left.is_true() {
                            Ok(LoxValue::Boolean(false))
                        } else {
                            if let Some(right) = right {
                                Ok(LoxValue::Boolean(self.evaluate(*right)?.is_true()))
                            } else {
                                Ok(LoxValue::Boolean(true))
                            }
                        }
                    }
                    _ => Err(LoxError::RuntimeError {
                        line: operator.line,
                        msg: format!("'{}' is not a valid logical operator", operator.lexeme),
                    }),
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(*callee)?;
                let mut args = vec![];
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }
                if let LoxValue::Callable(callable) = callee {
                    Ok(callable.call(self, args, paren.line)?)
                } else {
                    Err(LoxError::RuntimeError {
                        line: paren.line,
                        msg: "can only call function and clases".to_string(),
                    })
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
