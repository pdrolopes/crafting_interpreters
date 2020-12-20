use super::expr;
use super::expr::Expr;
use super::stmt;
use super::stmt::Stmt;
use super::token::Token;
use crate::error::{LoxError, Result};
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum VarState {
    Declared,
    Defined,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, VarState>>,
    expr_id_scope_depth: HashMap<u64, u64>,
}
impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: vec![HashMap::new()],
            expr_id_scope_depth: HashMap::new(),
        }
    }
    pub fn run(mut self, statements: &[Stmt]) -> Result<HashMap<u64, u64>> {
        self.resolve_stmts(statements)?;
        Ok(self.expr_id_scope_depth)
    }
    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        expr.accept(self)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn resolve_stmts(&mut self, stmts: &[Stmt]) -> Result<()> {
        stmts
            .into_iter()
            .map(|stmt| self.resolve_stmt(stmt))
            .collect()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    fn declare(&mut self, token: &Token) {
        self.scopes
            .iter_mut()
            .last()
            .and_then(|map| map.insert(token.lexeme.clone(), VarState::Declared));
    }
    fn define(&mut self, token: &Token) {
        self.scopes
            .iter_mut()
            .last()
            .and_then(|map| map.insert(token.lexeme.clone(), VarState::Defined));
    }

    fn resolve_local(&mut self, token: &Token, expr_id: u64) {
        let scope_size = self.scopes.len() as u64;
        let found_index = self
            .scopes
            .iter()
            .rposition(|scope| scope.get(&token.lexeme).is_some())
            .map(|index| index as u64);

        if let Some(found_index) = found_index {
            self.expr_id_scope_depth
                .insert(expr_id, scope_size - 1 - found_index);
        }
    }
    fn resolve_function(&mut self, params: &[Token], body: &[Stmt]) -> Result<()> {
        self.begin_scope();
        params.into_iter().for_each(|param| {
            self.declare(param);
            self.define(param);
        });
        self.resolve_stmts(body)?;
        self.end_scope();
        Ok(())
    }
}
impl stmt::Visitor<Result<()>> for Resolver {
    fn visit_block_stmt(&mut self, statements: &[stmt::Stmt]) -> Result<()> {
        self.begin_scope();
        self.resolve_stmts(statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expr: &expr::Expr) -> Result<()> {
        self.resolve_expr(expr)
    }

    fn visit_print_stmt(&mut self, expr: &expr::Expr) -> Result<()> {
        self.resolve_expr(expr)
    }

    fn visit_var_stmt(
        &mut self,
        token: &crate::token::Token,
        expr: Option<&expr::Expr>,
    ) -> Result<()> {
        self.declare(token);
        if let Some(expr) = expr {
            self.resolve_expr(expr)?;
        }
        self.define(token);
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        cond: &expr::Expr,
        then_branch: &stmt::Stmt,
        else_branch: Option<&stmt::Stmt>,
    ) -> Result<()> {
        self.resolve_expr(cond)?;
        self.resolve_stmt(then_branch)?;

        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, cond: &expr::Expr, block: &stmt::Stmt) -> Result<()> {
        self.resolve_expr(cond)?;
        self.resolve_stmt(block)
    }

    fn visit_function_stmt(
        &mut self,
        token: &crate::token::Token,
        params: &[crate::token::Token],
        body: &[stmt::Stmt],
    ) -> Result<()> {
        self.declare(token);
        self.define(token);
        self.resolve_function(params, body)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, expr: &expr::Expr) -> Result<()> {
        self.resolve_expr(expr)
    }
}
impl expr::Visitor<Result<()>> for Resolver {
    fn visit_binary_expr(
        &mut self,
        left: &expr::Expr,
        _: &crate::token::Token,
        right: &expr::Expr,
    ) -> Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Expr) -> Result<()> {
        self.resolve_expr(expr)
    }

    fn visit_unary_expr(&mut self, _: &crate::token::Token, expr: &expr::Expr) -> Result<()> {
        self.resolve_expr(expr)
    }

    fn visit_call_expr(
        &mut self,
        callee: &expr::Expr,
        _: &crate::token::Token,
        args: &[expr::Expr],
    ) -> Result<()> {
        self.resolve_expr(callee)?;
        args.into_iter().map(|arg| self.resolve_expr(arg)).collect()
    }

    fn visit_conditional_expr(
        &mut self,
        cond: &expr::Expr,
        then_branch: &expr::Expr,
        else_branch: &expr::Expr,
    ) -> Result<()> {
        self.resolve_expr(cond)?;
        self.resolve_expr(then_branch)?;
        self.resolve_expr(else_branch)
    }

    fn visit_literal_expr_number(&mut self, _: f64) -> Result<()> {
        Ok(())
    }

    fn visit_literal_expr_string(&mut self, _: &str) -> Result<()> {
        Ok(())
    }

    fn visit_literal_expr_boolean(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn visit_literal_expr_nil(&mut self) -> Result<()> {
        Ok(())
    }

    fn visit_variable_expr(&mut self, token: &crate::token::Token, id: u64) -> Result<()> {
        if self
            .scopes
            .iter()
            .last()
            .and_then(|map| map.get(&token.lexeme.clone()))
            .map(|state| *state == VarState::Declared)
            .unwrap_or(false)
        {
            return Err(LoxError::ResolverError(
                token.clone(),
                "Can't read local variable in its own initializer.".to_string(),
            ));
        }

        self.resolve_local(token, id);
        Ok(())
    }

    fn visit_assign_expr(
        &mut self,
        token: &crate::token::Token,
        expr: &expr::Expr,
        id: u64,
    ) -> Result<()> {
        self.resolve_expr(expr)?;
        self.resolve_local(token, id);
        Ok(())
    }

    fn visit_logic_or(&mut self, left: &expr::Expr, right: &expr::Expr) -> Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn visit_logic_and(&mut self, left: &expr::Expr, right: &expr::Expr) -> Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }
}
