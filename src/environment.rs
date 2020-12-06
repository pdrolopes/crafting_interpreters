use crate::error::{LoxError, Result};
use crate::object::Object;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Object) {
        self.variables.insert(key, value);
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<()> {
        if self.variables.contains_key(&token.lexeme) {
            self.variables.insert(token.lexeme.clone(), value);
            return Ok(());
        }

        Err(LoxError::RuntimeError(
            token.clone(),
            format!("Undefined variable, `{}`", token.lexeme),
        ))
    }

    pub fn get(&self, name: &Token) -> Result<&Object> {
        self.variables.get(&name.lexeme).ok_or_else(|| {
            LoxError::RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            )
        })
    }
}
