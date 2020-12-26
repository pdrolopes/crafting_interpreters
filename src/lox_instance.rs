use crate::environment::Environment;
use crate::error::LoxError;
use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::lox_callable::Callable;
use crate::lox_class::LoxClass;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::Object;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        LoxInstance {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object> {
        self.fields.get(&token.lexeme).cloned().ok_or_else(|| {
            LoxError::RuntimeError(
                token.clone(),
                format!("Undefined property '{}'", token.lexeme),
            )
        })
    }
    pub fn set(&mut self, token: Token, value: Object) {
        self.fields.insert(token.lexeme, value);
    }
}
impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name())
    }
}
