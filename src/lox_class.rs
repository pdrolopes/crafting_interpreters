use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::interpreter::UserFunction;
use crate::lox_callable::Callable;
use crate::lox_instance::LoxInstance;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LoxClass {
    name: Token,
    methods: HashMap<String, UserFunction>,
}

impl LoxClass {
    pub fn new(name: Token, methods: HashMap<String, UserFunction>) -> Self {
        Self { name, methods }
    }

    pub fn name(&self) -> &str {
        &self.name.lexeme
    }

    pub fn find_method(&self, name: &str) -> Option<UserFunction> {
        self.methods.get(name).cloned()
    }
}
impl Callable for LoxClass {
    fn arity(&self) -> usize {
        self.find_method("init")
            .map(|method| method.arity())
            .unwrap_or(0)
    }

    fn call(&self, arguments: &[Object], interpreter: &mut Interpreter) -> Result<Object> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));

        self.find_method("init").map(|method| {
            method
                .bind(Rc::clone(&instance))
                .call(arguments, interpreter)
        });

        Ok(Object::ClassInstance(instance))
    }
}
