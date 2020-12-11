use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Stmt::Block(statements) => visitor.visit_block_stmt(statements),
            Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Var(token, expr) => visitor.visit_var_stmt(token, expr.as_ref()),
        }
    }
}

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> T;
}
