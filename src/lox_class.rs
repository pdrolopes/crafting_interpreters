use crate::environment::Environment;
use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::lox_callable::Callable;
use crate::lox_instance::LoxInstance;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

type Function = (Token, Vec<Token>, Vec<Stmt>);
#[derive(Clone, Debug)]
pub struct LoxClass {
    name: Token,
    methods: Vec<Function>,
}

impl LoxClass {
    pub fn new(name: Token, methods: Vec<Function>) -> Self {
        Self { name, methods }
    }

    pub fn name(&self) -> &str {
        &self.name.lexeme
    }
}
impl Callable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, arguments: &[Object], interpreter: &mut Interpreter) -> Result<Object> {
        Ok(Object::ClassInstance(Rc::new(RefCell::new(
            LoxInstance::new(self.clone()),
        ))))
    }
}
