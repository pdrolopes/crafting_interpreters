use super::token::Token;
use super::token_type::TokenType;

#[derive(Clone)]
enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Token),
}

impl Expr {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Binary(left, token, right) => {
                visitor.visit_binary_expr(left.as_ref(), token, right.as_ref())
            }
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr.as_ref()),
            Expr::Unary(token, expr) => visitor.visit_unary_expr(token, expr.as_ref()),
            Expr::Literal(token) => visitor.visit_literal_expr(token),
        }
    }
}

trait Visitor<T> {
    fn visit_binary_expr(&self, left: &Expr, token: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expr: &Expr) -> T;
    fn visit_unary_expr(&self, token: &Token, expr: &Expr) -> T;
    fn visit_literal_expr(&self, token: &Token) -> T;
}

struct ASTPrinter {}
impl ASTPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = format!("({}", name);

        for expr in exprs {
            let inner = format!(" {}", expr.accept(self));
            builder.push_str(&inner);
        }
        builder.push_str(")");

        builder
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_binary_expr(&self, left: &Expr, token: &Token, right: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[left, right])
    }
    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        self.parenthesize("Group", &[expr])
    }
    fn visit_unary_expr(&self, token: &Token, expr: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[expr])
    }
    fn visit_literal_expr(&self, token: &Token) -> String {
        match &token.kind {
            TokenType::String(value) => value.into(),
            TokenType::Number(value) => value.to_string(),
            _ => "nil".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_parser() {
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(1.0),
                "1".into(),
                0,
            ))),
            Token::new(TokenType::Plus, "+".into(), 0),
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(2.0),
                "1".into(),
                0,
            ))),
        );

        let output = ASTPrinter {}.print(&expr);
        assert_eq!(output, "(+ 1 2)");
    }

    #[test]
    fn test_other_expr_parser() {
        let unary_expr = Expr::Unary(
            Token::new(TokenType::Minus, "-".into(), 0),
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(123.0),
                "123".into(),
                0,
            ))),
        );

        let grouping_expr = Expr::Grouping(Box::new(Expr::Literal(Token::new(
            TokenType::Number(45.67),
            "45.67".into(),
            0,
        ))));
        let expr = Expr::Binary(
            Box::new(unary_expr),
            Token::new(TokenType::Star, "*".into(), 0),
            Box::new(grouping_expr),
        );

        let output = ASTPrinter {}.print(&expr);
        assert_eq!(output, "(* (- 123) (Group 45.67))");
    }
}
