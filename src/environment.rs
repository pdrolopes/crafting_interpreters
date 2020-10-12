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

    pub fn get(&self, name: &Token) -> Result<&Object> {
        self.variables.get(&name.lexeme).ok_or_else(|| {
            LoxError::RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            )
        })
    }
}
