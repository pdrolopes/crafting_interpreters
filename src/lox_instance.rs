use crate::error::LoxError;
use crate::error::Result;
use crate::lox_class::LoxClass;
use crate::token::Token;
use crate::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

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

    pub fn get(instance: Rc<RefCell<LoxInstance>>, token: &Token) -> Result<Object> {
        let instance_borrow = instance.borrow();
        instance_borrow
            .fields
            .get(&token.lexeme)
            .cloned()
            .or_else(|| {
                instance_borrow
                    .class
                    .find_method(&token.lexeme)
                    .map(|method| method.bind(Rc::clone(&instance)))
                    .map(|m| Object::Call(Box::new(m)))
            })
            .ok_or_else(|| {
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
