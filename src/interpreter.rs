use super::expr;
use super::expr::Expr;
use super::stmt;
use super::stmt::Stmt;
use crate::environment::Environment;
use crate::error::{LoxError, Result};
use crate::lox;
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
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
}

fn is_truphy(object: Object) -> bool {
    match object {
        Object::Boolean(x) => x,
        Object::Nil => false,
        _ => true,
    }
}

impl expr::Visitor<Result<Object>> for Interpreter {
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> Result<Object> {
        // TODO probably only evaluate right when necessary
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
            (TokenType::Bang, x) => Ok(Object::Boolean(!is_truphy(x))),
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
        if is_truphy(cond) {
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

    fn visit_variable_expr(&mut self, token: &Token) -> Result<Object> {
        self.environment
            .borrow()
            .get(token)
            .map(|object| object.clone())
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<Object> {
        let object = self.evaluate(expr)?;
        self.environment
            .borrow_mut()
            .assign(token, object.clone())?;

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
}

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &[stmt::Stmt]) -> Result<()> {
        let mut enclosed_enviroment = Rc::new(RefCell::new(Environment::new_with_enclosing(
            Rc::clone(&self.environment),
        )));

        // Ugly code where environment is swapped out to the new enclosed enviroment.
        // After statements are executed. I swap it back
        std::mem::swap(&mut self.environment, &mut enclosed_enviroment);

        let results: Result<()> = statements
            .into_iter()
            .map(|stmt| self.execute(stmt))
            .collect();

        std::mem::swap(&mut self.environment, &mut enclosed_enviroment);

        results
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<()> {
        self.evaluate(expr)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;

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
        // let value = self.evaluate(expr)?;

        self.environment
            .borrow_mut()
            .define(token.lexeme.clone(), value);

        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        cond: &Expr,
        then_branch: &Box<Stmt>,
        else_branch: Option<&Box<Stmt>>,
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
}
