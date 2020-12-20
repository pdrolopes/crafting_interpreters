use super::expr::{Expr, Visitor};
use super::token::Token;

pub struct ASTPrinter;

impl ASTPrinter {
    pub fn print(expr: &Expr) -> String {
        let mut printer = ASTPrinter {};
        expr.accept(&mut printer)
    }
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
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
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[left, right])
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        self.parenthesize("Group", &[expr])
    }
    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[expr])
    }

    fn visit_literal_expr_number(&mut self, value: f64) -> String {
        value.to_string()
    }

    fn visit_literal_expr_string(&mut self, value: &str) -> String {
        value.into()
    }

    fn visit_literal_expr_boolean(&mut self, value: bool) -> String {
        value.to_string()
    }

    fn visit_literal_expr_nil(&mut self) -> String {
        "nil".into()
    }

    fn visit_conditional_expr(
        &mut self,
        cond: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> String {
        self.parenthesize("Cond", &[cond, then_branch, else_branch])
    }

    fn visit_variable_expr(&mut self, token: &Token) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> String {
        todo!()
    }

    fn visit_logic_or(&mut self, left: &Expr, right: &Expr) -> String {
        todo!()
    }

    fn visit_logic_and(&mut self, left: &Expr, right: &Expr) -> String {
        todo!()
    }

    fn visit_call_expr(&mut self, callee: &Expr, token: &Token, args: &[Expr]) -> String {
        todo!()
    }
}

// --- Reverse Polish Notation ---
struct RPNPrinter {}
impl RPNPrinter {
    fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
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
    fn visit_binary_expr(&mut self, left: &Expr, token: &Token, right: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[left, right])
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        self.parenthesize("Group", &[expr])
    }
    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> String {
        self.parenthesize(&token.lexeme, &[expr])
    }
    fn visit_literal_expr_number(&mut self, value: f64) -> String {
        value.to_string()
    }

    fn visit_literal_expr_string(&mut self, value: &str) -> String {
        value.into()
    }

    fn visit_literal_expr_boolean(&mut self, value: bool) -> String {
        value.to_string()
    }

    fn visit_literal_expr_nil(&mut self) -> String {
        "nil".into()
    }

    fn visit_conditional_expr(
        &mut self,
        cond: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> String {
        todo!()
    }

    fn visit_variable_expr(&mut self, token: &Token) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> String {
        todo!()
    }

    fn visit_logic_or(&mut self, left: &Expr, right: &Expr) -> String {
        todo!()
    }

    fn visit_logic_and(&mut self, left: &Expr, right: &Expr) -> String {
        todo!()
    }

    fn visit_call_expr(&mut self, callee: &Expr, token: &Token, args: &[Expr]) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token_type::TokenType;

    #[test]
    fn test_expr_parser() {
        let expr = Expr::Binary(
            Box::new(Expr::Number(1.0)),
            Token::new(TokenType::Plus, "+".into(), 0),
            Box::new(Expr::Number(2.0)),
        );

        let output = ASTPrinter::print(&expr);
        assert_eq!(output, "(+ 1 2)");
    }

    #[test]
    fn test_other_expr_parser() {
        let plus = Expr::Binary(
            Box::new(Expr::Number(1.0)),
            Token::new(TokenType::Plus, "+".into(), 0),
            Box::new(Expr::Number(2.0)),
        );

        let minus = Expr::Binary(
            Box::new(Expr::Number(4.0)),
            Token::new(TokenType::Minus, "-".into(), 0),
            Box::new(Expr::Number(3.0)),
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
