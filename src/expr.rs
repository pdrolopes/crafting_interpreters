use super::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>), // conditional - then - else,
    Call(Box<Expr>, Token, Vec<Expr>),

    // Variables
    Variable(Token, u64),
    Assign(Token, Box<Expr>, u64),
    LogicOr(Box<Expr>, Box<Expr>),
    LogicAnd(Box<Expr>, Box<Expr>),

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
            Expr::Call(callee, token, arguments) => {
                visitor.visit_call_expr(callee, token, arguments)
            }
            Expr::Conditional(expr, then_branch, else_branch) => visitor.visit_conditional_expr(
                expr.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ),
            Expr::Number(x) => visitor.visit_literal_expr_number(*x),
            Expr::String(x) => visitor.visit_literal_expr_string(x),
            Expr::Boolean(x) => visitor.visit_literal_expr_boolean(*x),
            Expr::Nil => visitor.visit_literal_expr_nil(),
            Expr::Variable(token, id) => visitor.visit_variable_expr(token, *id),
            Expr::Assign(token, expr, id) => visitor.visit_assign_expr(token, expr, *id),
            Expr::LogicOr(left, right) => visitor.visit_logic_or(left, right),
            Expr::LogicAnd(left, right) => visitor.visit_logic_and(left, right),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, token: &Token, args: &[Expr]) -> T;
    fn visit_conditional_expr(&mut self, cond: &Expr, then_branch: &Expr, else_branch: &Expr) -> T;
    fn visit_literal_expr_number(&mut self, value: f64) -> T;
    fn visit_literal_expr_string(&mut self, value: &str) -> T;
    fn visit_literal_expr_boolean(&mut self, value: bool) -> T;
    fn visit_literal_expr_nil(&mut self) -> T;
    fn visit_variable_expr(&mut self, token: &Token, id: u64) -> T;
    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr, id: u64) -> T;
    fn visit_logic_or(&mut self, left: &Expr, right: &Expr) -> T;
    fn visit_logic_and(&mut self, left: &Expr, right: &Expr) -> T;
}
