use super::expr;
use super::expr::Expr;
use super::stmt;
use super::stmt::Stmt;
use crate::environment::Environment;
use crate::error::{LoxError, Result};
use crate::lox;
use crate::lox_callable::Callable;
use crate::lox_class::LoxClass;
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    global_environment: Rc<RefCell<Environment>>,
    local_environment: Rc<RefCell<Environment>>,
    expr_id_scope_depth: HashMap<u64, u64>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global_environment = create_global_enviroment();
        let global_environment = Rc::new(RefCell::new(global_environment));
        Interpreter {
            local_environment: Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::clone(
                &global_environment,
            )))),
            global_environment,
            expr_id_scope_depth: HashMap::new(),
        }
    }

    pub fn add_expr_ids_depth(&mut self, mut map: HashMap<u64, u64>) {
        map.drain().for_each(|(key, value)| {
            self.expr_id_scope_depth.insert(key, value);
        });
    }

    pub fn environment(&self) -> Rc<RefCell<Environment>> {
        Rc::clone(&self.local_environment)
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            stmt.accept(self)
                .unwrap_or_else(|err| lox::report_runtime(err));
        }
    }

    pub fn print(&mut self, statement: &Stmt) {
        if let Stmt::Expression(x) = statement {
            stmt::Visitor::visit_print_stmt(self, x);
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        enclosing_environment: Environment,
    ) -> Result<()> {
        let mut enclosing_environment = Rc::new(RefCell::new(enclosing_environment));
        // Ugly code where environment is swapped out to the new enclosed enviroment.
        // After statements are executed. I swap it back
        std::mem::swap(&mut self.local_environment, &mut enclosing_environment);

        let results: Result<()> = statements
            .into_iter()
            .map(|stmt| self.execute(stmt))
            .collect();

        std::mem::swap(&mut self.local_environment, &mut enclosing_environment);

        results
    }
}

impl expr::Visitor<Result<Object>> for Interpreter {
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> Result<Object> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match (&token.kind, left, right) {
            //equality
            (TokenType::EqualEqual, left, right) => Ok(Object::Boolean(left == right)),
            (TokenType::BangEqual, left, right) => Ok(Object::Boolean(left != right)),

            // comparison
            // number comparison
            (TokenType::Greater, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Boolean(left > right))
            }
            (TokenType::GreaterEqual, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Boolean(left >= right))
            }
            (TokenType::Less, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Boolean(left < right))
            }
            (TokenType::LessEqual, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Boolean(left <= right))
            }

            // string comparison
            (TokenType::Greater, Object::String(left), Object::String(right)) => {
                Ok(Object::Boolean(left > right))
            }
            (TokenType::GreaterEqual, Object::String(left), Object::String(right)) => {
                Ok(Object::Boolean(left >= right))
            }
            (TokenType::Less, Object::String(left), Object::String(right)) => {
                Ok(Object::Boolean(left < right))
            }
            (TokenType::LessEqual, Object::String(left), Object::String(right)) => {
                Ok(Object::Boolean(left <= right))
            }
            (TokenType::Greater, _, _)
            | (TokenType::GreaterEqual, _, _)
            | (TokenType::Less, _, _)
            | (TokenType::LessEqual, _, _) => Err(LoxError::RuntimeError(
                token.clone(),
                "Expected operands to be numbers".into(),
            )),

            // addition
            (TokenType::Plus, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Number(left + right))
            }
            (TokenType::Plus, Object::String(left), Object::String(right)) => {
                Ok(Object::String(format!("{}{}", left, right)))
            }
            (TokenType::Plus, Object::Number(left), Object::String(right)) => {
                Ok(Object::String(format!("{}{}", left, right)))
            }
            (TokenType::Plus, Object::String(left), Object::Number(right)) => {
                Ok(Object::String(format!("{}{}", left, right)))
            }
            (TokenType::Minus, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Number(left - right))
            }
            (TokenType::Plus, _, _) => Err(LoxError::RuntimeError(
                token.clone(),
                "Expected operands to be numbers or strings".into(),
            )),
            (TokenType::Minus, _, _) => Err(LoxError::RuntimeError(
                token.clone(),
                "Expected operands to be numbers".into(),
            )),

            // multiplication
            (TokenType::Star, Object::Number(left), Object::Number(right)) => {
                if right == 0.0 {
                    Err(LoxError::RuntimeError(
                        token.clone(),
                        "Cannot divide by zero".into(),
                    ))
                } else {
                    Ok(Object::Number(left * right))
                }
            }
            (TokenType::Slash, Object::Number(left), Object::Number(right)) => {
                Ok(Object::Number(left / right))
            }

            (TokenType::Star, _, _) | (TokenType::Slash, _, _) => Err(LoxError::RuntimeError(
                token.clone(),
                "Expected operands to be numbers".into(),
            )),

            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> Result<Object> {
        let eval = self.evaluate(expr)?;
        match (&token.kind, eval) {
            (TokenType::Bang, x) => Ok(Object::Boolean(!x.is_truphy())),
            (TokenType::Minus, Object::Number(value)) => Ok(Object::Number(-value)),
            (TokenType::Minus, _) => Err(LoxError::RuntimeError(
                token.clone(),
                "Operand must be a number".into(),
            )),
            _ => unreachable!(),
        }
    }

    fn visit_conditional_expr(
        &mut self,
        cond: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<Object> {
        let cond = self.evaluate(cond)?;
        if cond.is_truphy() {
            self.evaluate(then_branch)
        } else {
            self.evaluate(else_branch)
        }
    }

    fn visit_literal_expr_number(&mut self, value: f64) -> Result<Object> {
        Ok(Object::Number(value))
    }

    fn visit_literal_expr_string(&mut self, value: &str) -> Result<Object> {
        Ok(Object::String(value.into()))
    }

    fn visit_literal_expr_boolean(&mut self, value: bool) -> Result<Object> {
        Ok(Object::Boolean(value))
    }

    fn visit_literal_expr_nil(&mut self) -> Result<Object> {
        Ok(Object::Nil)
    }

    fn visit_variable_expr(&mut self, token: &Token, id: u64) -> Result<Object> {
        let distance = self.expr_id_scope_depth.get(&id);

        match distance {
            Some(distance) => self.local_environment.borrow().get_at(token, *distance),
            None => self.global_environment.borrow().get(token),
        }
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr, id: u64) -> Result<Object> {
        let object = self.evaluate(expr)?;

        let distance = self.expr_id_scope_depth.get(&id);

        match distance {
            Some(distance) => {
                self.local_environment
                    .borrow_mut()
                    .assign_at(token, object.clone(), *distance)?
            }
            None => self
                .global_environment
                .borrow_mut()
                .assign(token, object.clone())?,
        };

        Ok(object)
    }

    fn visit_logic_or(&mut self, left: &Expr, right: &Expr) -> Result<Object> {
        let left = self.evaluate(left)?;

        if left.is_truphy() {
            Ok(left)
        } else {
            self.evaluate(right)
        }
    }

    fn visit_logic_and(&mut self, left: &Expr, right: &Expr) -> Result<Object> {
        let left = self.evaluate(left)?;

        if !left.is_truphy() {
            Ok(left)
        } else {
            self.evaluate(right)
        }
    }

    fn visit_call_expr(&mut self, callee: &Expr, token: &Token, args: &[Expr]) -> Result<Object> {
        let callee = self.evaluate(callee)?;

        let arguments: Result<Vec<Object>> =
            args.into_iter().map(|arg| self.evaluate(arg)).collect();
        let arguments = arguments?;

        let callable = if let Object::Call(callable) = callee {
            callable
        } else {
            return Err(LoxError::RuntimeError(
                token.clone(),
                "Can only calll on functions or classes".to_string(),
            ));
        };

        if callable.arity() != arguments.len() {
            return Err(LoxError::RuntimeError(
                token.clone(),
                format!(
                    "Expect {} arguments but found {}",
                    callable.arity(),
                    arguments.len()
                ),
            ));
        }
        callable.call(&arguments, self)
    }

    fn visit_get_expr(&mut self, object: &Expr, property: &Token) -> Result<Object> {
        let object = self.evaluate(object)?;

        let instance = if let Object::ClassInstance(instance) = object {
            instance
        } else {
            return Err(LoxError::RuntimeError(
                property.clone(),
                "Only instances have properties".to_string(),
            ));
        };

        let value = instance.borrow().get(property);
        value
    }

    fn visit_set_expr(&mut self, object: &Expr, property: &Token, value: &Expr) -> Result<Object> {
        let object = self.evaluate(object)?;

        let object = if let Object::ClassInstance(instance) = object {
            dbg!(instance)
        } else {
            return Err(LoxError::RuntimeError(
                property.clone(),
                "Only instances have fields".to_string(),
            ));
        };

        let value = self.evaluate(value)?;
        object.borrow_mut().set(property.clone(), value.clone());

        Ok(value)
    }
}

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &[stmt::Stmt]) -> Result<()> {
        let enclosed_enviroment = Environment::new_with_enclosing(self.environment());
        self.execute_block(statements, enclosed_enviroment)
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<()> {
        self.evaluate(expr)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;

        dbg!(&self.local_environment);
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> Result<()> {
        let value = expr.as_ref().map(|value| self.evaluate(value));

        let value = match value {
            Some(Err(x)) => return Err(x),
            Some(Ok(x)) => Some(x),
            None => None,
        };

        self.local_environment
            .borrow_mut()
            .define(token.lexeme.clone(), value);

        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<()> {
        let cond = self.evaluate(cond)?;

        if cond.is_truphy() {
            self.execute(then_branch)
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&mut self, cond: &Expr, block: &Stmt) -> Result<()> {
        while self.evaluate(cond)?.is_truphy() {
            self.execute(block)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> Result<()> {
        self.local_environment.borrow_mut().define(
            name.lexeme.clone(),
            Some(Object::Call(Box::new(UserFunction::new(
                Vec::from(params),
                Vec::from(body),
                self.environment(),
            )))),
        );
        Ok(())
    }

    fn visit_return_stmt(&mut self, token: &Token, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;
        Err(LoxError::Return(value))
    }

    fn visit_class_stmt(&mut self, token: &Token, methods: &[Stmt]) -> Result<()> {
        self.local_environment
            .borrow_mut()
            .define(token.lexeme.clone(), None);
        let class = LoxClass::new(token.clone(), vec![]);
        self.local_environment
            .borrow_mut()
            .assign(token, Object::Call(Box::new(class)))?;

        Ok(())
    }
}
fn create_global_enviroment() -> Environment {
    let mut global_environment = Environment::new();
    global_environment.define(
        "clock".to_string(),
        Some(Object::Call(Box::new(ClockFunction {}))),
    );

    global_environment
}

// global functions

#[derive(Clone, Debug)]
struct ClockFunction {}
impl Callable for ClockFunction {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &[Object], _: &mut Interpreter) -> Result<Object> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(Object::Number(since_the_epoch.as_secs_f64()))
    }
}

#[derive(Clone, Debug)]
struct UserFunction {
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}
impl UserFunction {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Self {
        UserFunction {
            params,
            body,
            closure: environment,
        }
    }
}
impl Callable for UserFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, arguments: &[Object], interpreter: &mut Interpreter) -> Result<Object> {
        let mut environment = Environment::new_with_enclosing(Rc::clone(&self.closure));

        self.params
            .iter()
            .zip(arguments)
            .for_each(|(param, argument)| {
                environment.define(param.lexeme.to_string(), Some(argument.clone()))
            });

        let result = interpreter.execute_block(&self.body, environment);

        match result {
            Ok(()) => Ok(Object::Nil),
            Err(LoxError::Return(value)) => Ok(value),
            Err(x) => Err(x),
        }
    }
}
