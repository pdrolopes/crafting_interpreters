use super::error;
use super::error::{LoxError, Result};
use super::expr::Expr;
use super::lox;
use super::token::Token;
use super::token_type::TokenType;
use crate::stmt::Stmt;
use std::iter::Peekable;
use std::slice::Iter;

const MAX_FUN_ARGUMENTS: usize = 255;
pub struct Parser<'a> {
    tokens_iter: Peekable<Iter<'a, Token>>,
    allow_only_expression: bool,
    found_only_expr: bool, // flag that signals if a expression only was found(without ending ;)
}

#[derive(Clone)]
pub enum ParseResult {
    List(Vec<Result<Stmt>>),
    SingleExpr(Result<Stmt>),
}

#[derive(Debug)]
pub enum FunctionKind {
    Function,
    Method,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], allow_only_expression: bool) -> Self {
        Self {
            tokens_iter: tokens.iter().peekable(),
            allow_only_expression,
            found_only_expr: false,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut parsed_list = Vec::new();

        while let Some(token) = self.tokens_iter.peek() {
            if token.kind == TokenType::Eof {
                break;
            }

            let declaration = self.declaration();

            if self.found_only_expr {
                return ParseResult::SingleExpr(declaration);
            }

            parsed_list.push(declaration);
        }

        ParseResult::List(parsed_list)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        let result = if self
            .tokens_iter
            .next_if(|token| token.kind == TokenType::Fun)
            .is_some()
        {
            self.fun_declaration(FunctionKind::Function)
        } else if self
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

    fn fun_declaration(&mut self, kind: FunctionKind) -> Result<Stmt> {
        let token_name = self
            .consume(TokenType::Identifier, &format!("Expected {:?} name", kind))?
            .clone();
        self.consume(
            TokenType::LeftParen,
            &format!("Expected '(' after {:?}", kind),
        )?;

        let mut parameters = vec![];

        if self
            .tokens_iter
            .peek()
            .map(|token| token.kind != TokenType::RightParen)
            .unwrap_or(false)
        {
            loop {
                if parameters.len() > MAX_FUN_ARGUMENTS {
                    return Err(LoxError::RuntimeError(
                        token_name,
                        "Reached maximum number of parameters(255)".to_string(),
                    ));
                }
                let param = self
                    .consume(TokenType::Identifier, "Expected identifier")?
                    .clone();
                parameters.push(param);

                if self
                    .tokens_iter
                    .next_if(|token| token.kind == TokenType::Comma)
                    .is_none()
                {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            &format!("Expected ')' after {:?} parameters.", kind),
        )?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expected '{{' before {:?} body.", kind),
        )?;
        let body = match self.block()? {
            Stmt::Block(statements) => statements.clone(),
            x => vec![x],
        };

        Ok(Stmt::Function(token_name, parameters, body))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name")?
            .clone();

        let mut initializer = None;
        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Equal)
            .is_some()
        {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ; after variable declaration")?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::If)
            .is_some()
        {
            return self.if_stmt();
        }

        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Print)
            .is_some()
        {
            return self.print_stmt();
        }

        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::While)
            .is_some()
        {
            return self.while_stmt();
        }

        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::For)
            .is_some()
        {
            return self.for_stmt();
        }

        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::LeftBrace)
            .is_some()
        {
            return self.block();
        }

        if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Return)
            .is_some()
        {
            return self.return_stmt();
        }

        self.expr_stmt()
    }

    fn if_stmt(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "expected '(' after if")?;
        let cond = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "expected ')' to close if conditional",
        )?;

        let then_branch = self.statement()?;
        let else_branch = if self
            .tokens_iter
            .next_if(|t| t.kind == TokenType::Else)
            .is_some()
        {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(cond, Box::new(then_branch), else_branch))
    }

    fn expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;

        let next_token_is_EOF = self
            .tokens_iter
            .peek()
            .map(|token| matches!(token.kind, TokenType::Eof))
            .unwrap_or(false);
        if self.allow_only_expression && next_token_is_EOF {
            self.found_only_expr = true;
            return Ok(Stmt::Expression(expr));
        } else {
            self.consume(TokenType::Semicolon, "Expected ; after expression")?;
        }

        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut statements = vec![];

        while self
            .tokens_iter
            .peek()
            .and_then(|token| {
                if !matches!(&token.kind, TokenType::RightBrace | TokenType::Eof) {
                    Some(token)
                } else {
                    None
                }
            })
            .is_some()
        {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ; after value")?;

        Ok(Stmt::Print(expr))
    }

    fn while_stmt(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expected '(' before condition")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition")?;

        let block = self.statement()?;

        Ok(Stmt::While(cond, Box::new(block)))
    }

    fn return_stmt(&mut self) -> Result<Stmt> {
        let expr = if self
            .tokens_iter
            .peek()
            .map(|t| t.kind != TokenType::Semicolon)
            .unwrap_or(false)
        {
            self.expression()?
        } else {
            Expr::Nil
        };

        self.consume(TokenType::Semicolon, "Expected ; after return expression")?;

        Ok(Stmt::Return(expr))
    }

    fn for_stmt(&mut self) -> Result<Stmt> {
        // desugar for into while
        self.consume(TokenType::LeftParen, "Expected '(' after for")?;
        let initializer = match self
            .tokens_iter
            .next_if(|token| matches!(token.kind, TokenType::Semicolon | TokenType::Var))
            .map(|token| &token.kind)
        {
            Some(TokenType::Var) => Some(self.var_declaration()?),
            Some(TokenType::Semicolon) => None,
            _ => Some(self.expr_stmt()?),
        };

        let condition = if let Some(TokenType::Semicolon) = self.tokens_iter.peek().map(|t| &t.kind)
        {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expected ';' after conditional")?;

        let increment =
            if let Some(TokenType::RightParen) = self.tokens_iter.peek().map(|t| &t.kind) {
                None
            } else {
                Some(self.expression()?)
            };

        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;

        let mut block = self.statement()?;

        if let Some(increment) = increment {
            block = Stmt::Block(vec![block, Stmt::Expression(increment)]);
        }

        if let Some(condition) = condition {
            block = Stmt::While(condition, Box::new(block));
        } else {
            block = Stmt::While(Expr::Boolean(true), Box::new(block));
        }

        if let Some(initializer) = initializer {
            block = Stmt::Block(vec![initializer, block]);
        }

        Ok(block)
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
        let expr = self.logic_or()?;

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

    fn logic_or(&mut self) -> Result<Expr> {
        let mut left = self.logic_and()?;

        while self
            .tokens_iter
            .next_if(|token| token.kind == TokenType::Or)
            .is_some()
        {
            let right = self.logic_and()?;
            left = Expr::LogicOr(Box::new(left), Box::new(right));
        }

        return Ok(left);
    }

    fn logic_and(&mut self) -> Result<Expr> {
        let mut left = self.equality()?;

        while self
            .tokens_iter
            .next_if(|token| token.kind == TokenType::And)
            .is_some()
        {
            let right = self.equality()?;
            left = Expr::LogicAnd(Box::new(left), Box::new(right))
        }

        return Ok(left);
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
            let right = self.call()?;
            Ok(Expr::Unary(operator.clone(), Box::new(right)))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self
                .tokens_iter
                .next_if(|token| token.kind == TokenType::LeftParen)
                .is_some()
            {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr> {
        let mut arguments = vec![];

        if self
            .tokens_iter
            .peek()
            .map(|token| token.kind != TokenType::RightParen)
            .unwrap_or(false)
        {
            loop {
                let argument = self.expression()?;
                arguments.push(argument);
                if arguments.len() > MAX_FUN_ARGUMENTS {
                    return Err(error(
                        (*self.tokens_iter.peek().unwrap()).clone(),
                        "Too many arguments",
                    ));
                }
                if self
                    .tokens_iter
                    .next_if(|token| token.kind == TokenType::Comma)
                    .is_none()
                {
                    break;
                }
            }
        }

        let paren_token = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;

        Ok(Expr::Call(Box::new(expr), paren_token.clone(), arguments))
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
