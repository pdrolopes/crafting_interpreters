use super::expr::{Expr, Visitor};
use crate::error::{LoxError, Result};
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;

struct Interpreter;

pub fn interpret(expr: &Expr) -> Result<Object> {
    Interpreter {}.evaluate(expr)
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }
}
fn is_truphy(object: Object) -> bool {
    match object {
        Object::Boolean(x) => x,
        Object::Nil => false,
        _ => true,
    }
}

impl Visitor<Result<Object>> for Interpreter {
    fn visit_binary_expr(&self, left: &Expr, token: &Token, right: &Expr) -> Result<Object> {
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

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<Object> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&self, token: &Token, expr: &Expr) -> Result<Object> {
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
        &self,
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

    fn visit_literal_expr_number(&self, value: f64) -> Result<Object> {
        Ok(Object::Number(value))
    }

    fn visit_literal_expr_string(&self, value: &str) -> Result<Object> {
        Ok(Object::String(value.into()))
    }

    fn visit_literal_expr_boolean(&self, value: bool) -> Result<Object> {
        Ok(Object::Boolean(value))
    }

    fn visit_literal_expr_nil(&self) -> Result<Object> {
        Ok(Object::Nil)
    }
}
