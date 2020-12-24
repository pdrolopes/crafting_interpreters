use crate::error::{LoxError, Result};
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Option<Object>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            variables: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, key: String, value: Option<Object>) {
        self.variables.insert(key, value);
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<()> {
        if self.variables.contains_key(&token.lexeme) {
            self.variables.insert(token.lexeme.clone(), Some(value));
            return Ok(());
        }

        Err(LoxError::RuntimeError(
            token.clone(),
            format!("Undefined variable, `{}`", token.lexeme),
        ))
    }

    pub fn assign_at(&mut self, token: &Token, value: Object, distance: u64) -> Result<()> {
        match distance {
            0 => self.assign(token, value),
            distance => self.enclosing.as_ref().expect("Expected to enviroment have an eclosing environment based on calculated distance").borrow_mut().assign_at(token, value, distance - 1),
        }
    }
    pub fn get_at(&self, token: &Token, distance: u64) -> Result<Object> {
        match distance {
            0 => self.get(token),
            distance => self.enclosing.as_ref().expect("Expected to enviroment have an eclosing environment based on calculated distance").borrow().get_at(token, distance - 1),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object> {
        let variable = self.variables.get(&token.lexeme).cloned();
        match variable {
            // Variable declared (initialized or not)
            Some(x) => x.ok_or_else(|| {
                LoxError::RuntimeError(
                    token.clone(),
                    format!("Non initialized variable '{}'.", token.lexeme),
                )
            }),
            // Non declared variable
            None => Err(LoxError::RuntimeError(
                token.clone(),
                format!("Undefined variable '{}'.", token.lexeme),
            )),
        }
    }
}
