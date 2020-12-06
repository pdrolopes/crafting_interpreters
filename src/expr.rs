use super::token::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>), // conditional - then - else,

    // Variables
    Variable(Token),
    Assign(Token, Box<Expr>),

    // Literal values
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expr::Binary(left, token, right) => {
                visitor.visit_binary_expr(left.as_ref(), token, right.as_ref())
            }
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr.as_ref()),
            Expr::Unary(token, expr) => visitor.visit_unary_expr(token, expr.as_ref()),
            Expr::Conditional(expr, then_branch, else_branch) => visitor.visit_conditional_expr(
                expr.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ),
            Expr::Number(x) => visitor.visit_literal_expr_number(*x),
            Expr::String(x) => visitor.visit_literal_expr_string(x),
            Expr::Boolean(x) => visitor.visit_literal_expr_boolean(*x),
            Expr::Nil => visitor.visit_literal_expr_nil(),
            Expr::Variable(token) => visitor.visit_variable_expr(token),
            Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> T;
    fn visit_conditional_expr(&mut self, cond: &Expr, then_branch: &Expr, else_branch: &Expr) -> T;
    fn visit_literal_expr_number(&mut self, value: f64) -> T;
    fn visit_literal_expr_string(&mut self, value: &str) -> T;
    fn visit_literal_expr_boolean(&mut self, value: bool) -> T;
    fn visit_literal_expr_nil(&mut self) -> T;
    fn visit_variable_expr(&mut self, token: &Token) -> T;
    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> T;
}
