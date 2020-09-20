use super::token::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),

    // Literal values
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Binary(left, token, right) => {
                visitor.visit_binary_expr(left.as_ref(), token, right.as_ref())
            }
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr.as_ref()),
            Expr::Unary(token, expr) => visitor.visit_unary_expr(token, expr.as_ref()),
            // Expr::Literal(token) => visitor.visit_literal_expr(token),
            Expr::Number(x) => visitor.visit_literal_expr_number(*x),
            Expr::String(x) => visitor.visit_literal_expr_string(x),
            Expr::Boolean(x) => visitor.visit_literal_expr_boolean(*x),
            Expr::Nil => visitor.visit_literal_expr_nil(),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, left: &Expr, token: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expr: &Expr) -> T;
    fn visit_unary_expr(&self, token: &Token, expr: &Expr) -> T;
    fn visit_literal_expr_number(&self, value: f64) -> T;
    fn visit_literal_expr_string(&self, value: &str) -> T;
    fn visit_literal_expr_boolean(&self, value: bool) -> T;
    fn visit_literal_expr_nil(&self) -> T;
}
