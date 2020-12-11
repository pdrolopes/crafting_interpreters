use crate::error::{LoxError, Result};
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Object>,
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

    pub fn define(&mut self, key: String, value: Object) {
        self.variables.insert(key, value);
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<()> {
        if self.variables.contains_key(&token.lexeme) {
            self.variables.insert(token.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(ref enclosing_enviroment) = self.enclosing {
            return enclosing_enviroment
                .try_borrow_mut()
                .expect("Not able to get mutable access to enviroment")
                .assign(token, value);
        }

        Err(LoxError::RuntimeError(
            token.clone(),
            format!("Undefined variable, `{}`", token.lexeme),
        ))
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        let get_value_from_enclosing = || {
            self.enclosing
                .as_ref()?
                .borrow()
                .get(name)
                .map(|value| value.clone())
                .ok()
        };

        self.variables
            .get(&name.lexeme)
            .map(|value| value.clone())
            .or_else(get_value_from_enclosing)
            .ok_or_else(|| {
                LoxError::RuntimeError(
                    name.clone(),
                    format!("Undefined variable '{}'.", name.lexeme),
                )
            })
    }
}
