use crate::{
    builtin_functions::declare_builtin_functions,
    callable::Callable,
    environment::{EnvRef, Environment},
    error::LoxError,
    expresssion::Expr,
    function::Function,
    lox_value::LoxValue,
    statement::Stmt,
    token::Token,
    token_type::TokenType,
};
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
enum FunctionType {
    Function,
    Method,
    #[default]
    None,
}
#[derive(Default, Debug, Clone)]
enum ClassType {
    Class,
    #[default]
    None,
}
#[derive(Default)]
pub struct Interpreter {
    pub globals: EnvRef,
    pub current_environment: EnvRef,
    pub locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    current_function_type: FunctionType,
    current_class_type: ClassType,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self::default();
        interpreter.current_environment = interpreter.globals.clone();
        declare_builtin_functions(&mut interpreter);
        interpreter
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        self.resolve(statements.clone())?;
        self.execute(statements)
    }
    pub fn execute(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }
    pub fn resolve(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
    pub fn execute_stmt(&mut self, statement: Stmt) -> Result<(), LoxError> {
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
                self.current_environment
                    .borrow_mut()
                    .define(name.lexeme, value);
            }
            Stmt::BlockStmt { statements } => {
                let previous_env = self.current_environment.clone();
                let block_env = Environment::new_enclosing(previous_env.clone());
                self.current_environment = block_env;
                let mut result = Ok(());
                for s in statements {
                    result = self.execute_stmt(s);
                    if result.is_err() {
                        break;
                    };
                }
                self.current_environment = previous_env;
                result?
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.evaluate(condition)?;
                if condition.is_true() {
                    self.execute_stmt(*then_branch)?
                } else if let Some(else_branch) = else_branch {
                    self.execute_stmt(*else_branch)?
                }
            }

            Stmt::WhileStmt { condition, body } => {
                while self.evaluate(condition.clone())?.is_true() {
                    self.execute_stmt(*body.clone())?;
                }
            }
            Stmt::FuncStmt { name, params, body } => {
                self.current_environment.borrow_mut().define(
                    name.lexeme.clone(),
                    LoxValue::Function(Function {
                        name: name.lexeme,
                        params: params.iter().map(|p| p.lexeme.clone()).collect(),
                        body,
                        closure: Some(self.current_environment.clone()),
                    }),
                );
            }
            Stmt::ClassStmt { name, methods } => {
                let mut methods_map = HashMap::new();
                for method in methods {
                    if let Stmt::FuncStmt { name, params, body } = method {
                        methods_map.insert(
                            name.lexeme.clone(),
                            Function {
                                name: name.lexeme,
                                params: params.iter().map(|p| p.lexeme.clone()).collect(),
                                body,
                                closure: Some(self.current_environment.clone()),
                            },
                        );
                    }
                }
                self.current_environment.borrow_mut().define(
                    name.lexeme.clone(),
                    LoxValue::Class(crate::class::Class {
                        name: name.lexeme,
                        methods: methods_map,
                    }),
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
            Expr::Identifier { name, id } => self.lookup(name, id),
            Expr::Assign { name, value, id } => {
                let value = self.evaluate(*value)?;
                if let Some(distance) = self.locals.get(&id) {
                    self.current_environment
                        .borrow_mut()
                        .assign_at(name, value, *distance)
                } else {
                    self.globals.borrow_mut().assign_at(name, value, 0)
                }
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

                if let LoxValue::Function(function) = callee {
                    Ok(function.call(self, args, paren.line)?)
                } else if let LoxValue::Class(class) = callee {
                    Ok(class.call(self, args, paren.line)?)
                } else {
                    Err(LoxError::RuntimeError {
                        line: paren.line,
                        msg: "can only call function, method or class".to_string(),
                    })
                }
            }
            Expr::Get { name, expr } => {
                let expr_value = self.evaluate(*expr)?;
                if let LoxValue::Instance(instance) = expr_value.clone() {
                    instance.borrow().get(&name, expr_value)
                } else {
                    Err(LoxError::GetError {
                        msg: format!("'{}' is not an instance of a class", name.lexeme),
                        line: name.line,
                    })
                }
            }
            Expr::Set {
                name,
                value,
                object,
            } => {
                if let LoxValue::Instance(instance) = self.evaluate(*object)? {
                    let value = self.evaluate(*value)?;
                    instance
                        .borrow_mut()
                        .fields
                        .insert(name.lexeme, value.clone());
                    Ok(value)
                } else {
                    Err(LoxError::GetError {
                        msg: String::from("unable to set property of invalid instance"),
                        line: name.line,
                    })
                }
            }
            Expr::This { keyword, id } => self.lookup(keyword, id),
        }
    }

    pub fn lookup(&self, name: Token, id: usize) -> Result<LoxValue, LoxError> {
        match self.locals.get(&id) {
            Some(distance) => self.current_environment.borrow().get_at(name, *distance),
            None => self.globals.borrow().get_at(name, 0),
        }
    }
    pub fn resolve_stmt(&mut self, stmt: Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::ExprStmt { expr } => {
                self.resolve_expr(expr)?;
            }
            Stmt::PrintStmt { expr } => {
                self.resolve_expr(expr)?;
            }
            Stmt::VarDeclStmt { name, initializer } => {
                self.declare(name.clone())?;
                self.resolve_expr(initializer)?;
                self.define(name);
            }
            Stmt::BlockStmt { statements } => {
                self.begin_scope();
                for statement in statements {
                    self.resolve_stmt(statement)?
                }
                self.end_scope();
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(*then_branch)?;
                if let Some(stmt) = else_branch {
                    self.resolve_stmt(*stmt)?;
                }
            }
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(*body)?;
            }
            Stmt::FuncStmt { name, params, body } => {
                self.resolve_function(name, body, params, FunctionType::Function)?;
            }
            Stmt::ClassStmt { name, methods } => {
                self.declare(name.clone())?;
                self.define(name);
                let current_class_type = self.current_class_type.clone();
                self.current_class_type = ClassType::Class;
                self.begin_scope();
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(String::from("this"), true);
                for method in methods {
                    if let Stmt::FuncStmt { name, params, body } = method {
                        self.resolve_function(name, body, params, FunctionType::Method)?;
                    }
                }
                self.end_scope();
                self.current_class_type = current_class_type;
            }
            Stmt::ReturnStmt { keyword, value } => {
                if let FunctionType::None = self.current_function_type {
                    return Err(LoxError::ResolveError {
                        line: keyword.line,
                        msg: String::from("can't use return outside function"),
                    });
                }
                if let Some(expr) = value {
                    self.resolve_expr(expr)?;
                }
            }
        }
        Ok(())
    }

    fn resolve_function(
        &mut self,
        name: Token,
        body: Vec<Stmt>,
        params: Vec<Token>,
        function_type: FunctionType,
    ) -> Result<(), LoxError> {
        self.declare(name.clone())?;
        self.define(name);
        let enclosing_fuction_type = self.current_function_type.clone();
        self.current_function_type = function_type;
        self.begin_scope();
        for param in params {
            self.declare(param.clone())?;
            self.define(param);
        }
        for stmt in body {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();
        self.current_function_type = enclosing_fuction_type;
        Ok(())
    }
    fn resolve_expr(&mut self, expr: Expr) -> Result<(), LoxError> {
        match expr {
            Expr::Literal { value: _ } => (),
            Expr::Unary { operator: _, right } => self.resolve_expr(*right)?,
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(*left)?;
                self.resolve_expr(*right)?;
            }
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(*left)?;
                if let Some(expr) = right {
                    self.resolve_expr(*expr)?;
                }
            }
            Expr::Grouping { expr } => self.resolve_expr(*expr)?,
            Expr::Identifier { name, id } => {
                if let Some(currrent_scope) = self.scopes.last()
                    && let Some(declared) = currrent_scope.get(&name.lexeme)
                    && !*declared
                {
                    return Err(LoxError::ResolveError {
                        line: name.line,
                        msg: String::from("Can't read local variable in its own initializer."),
                    });
                }
                self.resolve_local(id, name);
            }
            Expr::Assign { name, value, id } => {
                self.resolve_expr(*value)?;
                self.resolve_local(id, name);
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(*callee)?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Get { name, expr } => {
                self.declare(name.clone())?;
                self.define(name);
                self.resolve_expr(*expr)?;
            }
            Expr::Set {
                name,
                value,
                object,
            } => {
                self.declare(name.clone())?;
                self.define(name);
                self.resolve_expr(*value)?;
                self.resolve_expr(*object)?;
            }
            Expr::This { keyword, id } => match self.current_class_type {
                ClassType::Class => self.resolve_local(id, keyword),
                ClassType::None => {
                    return Err(LoxError::ResolveError {
                        line: keyword.line,
                        msg: String::from("can't use this keyword outside class"),
                    });
                }
            },
        }
        Ok(())
    }
    fn resolve_local(&mut self, id: usize, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        for (distance, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(id, distance);
                break;
            }
        }
    }
    fn define(&mut self, name: Token) {
        if let Some(current_scope) = self.scopes.last_mut()
            && current_scope.contains_key(&name.lexeme)
        {
            current_scope.insert(name.lexeme, true);
        }
    }
    fn declare(&mut self, name: Token) -> Result<(), LoxError> {
        if let Some(current_scope) = self.scopes.last_mut() {
            if current_scope.contains_key(&name.lexeme) {
                return Err(LoxError::ResolveError {
                    line: name.line,
                    msg: format!(
                        "identifier '{}' is already declared in this scope",
                        name.lexeme
                    ),
                });
            }
            current_scope.insert(name.lexeme, false);
        }
        Ok(())
    }
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
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
