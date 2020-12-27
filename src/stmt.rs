use crate::expr::Expr;
use crate::token::Token;

pub type Function = (Token, Vec<Token>, Vec<Stmt>);

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    While(Expr, Box<Stmt>),
    Return(Token, Expr),
    Class {
        token: Token,
        methods: Vec<Function>,
    },
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Stmt::Block(statements) => visitor.visit_block_stmt(statements),
            Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Var(token, expr) => visitor.visit_var_stmt(token, expr.as_ref()),
            Stmt::If(cond, then_branch, else_branch) => {
                visitor.visit_if_stmt(cond, then_branch, else_branch.as_deref())
            }
            Stmt::While(cond, block) => visitor.visit_while_stmt(cond, block),
            Stmt::Function(token, parameters, body) => {
                visitor.visit_function_stmt(token, parameters, body)
            }
            Stmt::Return(token, expr) => visitor.visit_return_stmt(token, expr),
            Stmt::Class { token, methods } => visitor.visit_class_stmt(token, methods),
        }
    }
}

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_var_stmt(&mut self, token: &Token, expr: Option<&Expr>) -> T;
    fn visit_if_stmt(&mut self, cond: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> T;
    fn visit_while_stmt(&mut self, cond: &Expr, block: &Stmt) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> T;
    fn visit_return_stmt(&mut self, token: &Token, expr: &Expr) -> T;
    fn visit_class_stmt(&mut self, token: &Token, methods: &[Function]) -> T;
}
