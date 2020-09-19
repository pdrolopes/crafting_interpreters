use super::expr::{Expr, Visitor};
use super::token::Token;
use super::token_type::TokenType;

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

// --- Reverse Polish Notation ---
struct RPNPrinter {}
impl RPNPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();

        for expr in exprs {
            let inner = format!("{} ", expr.accept(self));
            builder.push_str(&inner);
        }
        builder.push_str(name);

        builder
    }
}

impl Visitor<String> for RPNPrinter {
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
        let plus = Expr::Binary(
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(1.0),
                "1".into(),
                0,
            ))),
            Token::new(TokenType::Plus, "+".into(), 0),
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(2.0),
                "2".into(),
                0,
            ))),
        );

        let minus = Expr::Binary(
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(4.0),
                "4".into(),
                0,
            ))),
            Token::new(TokenType::Minus, "-".into(), 0),
            Box::new(Expr::Literal(Token::new(
                TokenType::Number(3.0),
                "3".into(),
                0,
            ))),
        );

        let mul = Expr::Binary(
            Box::new(plus),
            Token::new(TokenType::Star, "*".into(), 0),
            Box::new(minus),
        );

        let output = RPNPrinter {}.print(&mul);
        assert_eq!(output, "1 2 + 4 3 - *");
    }
}
