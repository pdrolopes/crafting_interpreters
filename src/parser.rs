use super::error;
use super::error::Result;
use super::expr::Expr;
use super::lox;
use super::token::Token;
use super::token_type::TokenType;
use crate::stmt::Stmt;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens_iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens_iter: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Result<Stmt>> {
        let mut parsed_list = Vec::new();

        while let Some(token) = self.tokens_iter.peek() {
            if token.kind == TokenType::Eof {
                break;
            }

            parsed_list.push(self.declaration());
        }

        parsed_list
    }

    fn declaration(&mut self) -> Result<Stmt> {
        let result = if self
            .tokens_iter
            .next_if(|token| token.kind == TokenType::Var)
            .is_some()
        {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Err(err) => {
                self.synchronize(); // walk until ;
                Err(err)
            }
            x => x,
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name")?
            .clone();

        let mut initializer = Expr::Nil;
        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Equal)
            .is_some()
        {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ; after variable declaration")?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Print)
            .is_some()
        {
            return self.print_stmt();
        }

        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ; after expression")?;

        Ok(Stmt::Expression(expr))
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ; after value")?;

        Ok(Stmt::Print(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.conditional()?;

        if let Some(equals) = self
            .tokens_iter
            .next_if(|token| token.kind == TokenType::Equal)
        {
            let value = self.conditional()?;

            if let Expr::Variable(token) = expr {
                return Ok(Expr::Assign(token, Box::new(value)));
            }

            error(equals.clone(), "Invalid assignment target");
        }

        return Ok(expr);
    }
    fn conditional(&mut self) -> Result<Expr> {
        let expr = self.equality()?;

        let kind = self.tokens_iter.peek().map(|t| &t.kind);

        if let Some(TokenType::Question) = kind {
            self.tokens_iter.next().unwrap();
            let then_branch = self.expression()?;
            self.consume(
                TokenType::Colon,
                "Expect ':' after then branch of conditional expression",
            )?;
            let else_branch = self.conditional()?;

            Ok(Expr::Conditional(
                Box::new(expr),
                Box::new(then_branch),
                Box::new(else_branch),
            ))
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        loop {
            let kind = self.tokens_iter.peek().map(|t| &t.kind);
            match kind {
                Some(TokenType::BangEqual) | Some(TokenType::EqualEqual) => {
                    let operator = self.tokens_iter.next().unwrap(); // Can unwrap safely because of peek
                    let right = self.comparison()?;
                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.addition()?;

        loop {
            let kind = self.tokens_iter.peek().map(|t| &t.kind);
            match kind {
                Some(TokenType::Greater)
                | Some(TokenType::GreaterEqual)
                | Some(TokenType::Less)
                | Some(TokenType::LessEqual) => {
                    let operator = self.tokens_iter.next().unwrap();
                    let right = self.addition()?;
                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr> {
        let mut expr = self.multiplication()?;

        loop {
            let kind = self.tokens_iter.peek().map(|t| &t.kind);
            match kind {
                Some(TokenType::Plus) | Some(TokenType::Minus) => {
                    let operator = self.tokens_iter.next().unwrap();
                    let right = self.multiplication()?;
                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        loop {
            let kind = self.tokens_iter.peek().map(|t| &t.kind);
            match kind {
                Some(TokenType::Slash) | Some(TokenType::Star) => {
                    let operator = self.tokens_iter.next().unwrap();
                    let right = self.unary()?;
                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        let kind = self.tokens_iter.peek().map(|t| &t.kind);
        let matches = matches!(kind, Some(TokenType::Bang) | Some(TokenType::Minus));

        if matches {
            let operator = self.tokens_iter.next().unwrap(); // safe unwrap
            let right = self.unary()?;
            Ok(Expr::Unary(operator.clone(), Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.tokens_iter.next() {
            Some(token) => match &token.kind {
                TokenType::False => Ok(Expr::Boolean(false)),
                TokenType::True => Ok(Expr::Boolean(true)),
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::Number(value) => Ok(Expr::Number(*value)),
                TokenType::String(value) => Ok(Expr::String(value.to_string())),
                TokenType::Identifier => Ok(Expr::Variable(token.clone())),
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expect ')' after expression")?;
                    Ok(Expr::Grouping(Box::new(expr)))
                }
                _ => Err(error((*token).clone(), "expected expression")),
            },
            None => todo!(),
        }
    }

    // --- helper functions ---
    fn consume(&mut self, token_type: TokenType, error_message: &str) -> error::Result<&Token> {
        if let Some(token) = self.tokens_iter.peek() {
            if token.kind == token_type {
                return Ok(self.tokens_iter.next().unwrap());
            }

            let err = error((**token).clone(), error_message);
            return Err(err);
        }

        todo!()
    }

    fn synchronize(&mut self) {
        let should_consume = |token: &'_ &Token| {
            token.kind == TokenType::Semicolon
                || !matches!(
                    token.kind,
                    TokenType::Class
                        | TokenType::Fun
                        | TokenType::Var
                        | TokenType::For
                        | TokenType::If
                        | TokenType::While
                        | TokenType::Print
                        | TokenType::Return
                )
        };
        while let Some(token) = self.tokens_iter.next_if(should_consume) {
            if token.kind == TokenType::Semicolon {
                break;
            }
        }
    }
}

fn error(token: Token, message: &str) -> error::LoxError {
    let line = token.line;
    lox::error_token(token, message);
    error::LoxError::ParserError(line, message.to_string())
}
