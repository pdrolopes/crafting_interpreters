use crate::lox_callable::Callable;
use crate::lox_instance::LoxInstance;
use core::fmt::Debug;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),
    String(String),
    Number(f64),
    Call(Box<dyn Callable>),
    ClassInstance(Rc<RefCell<LoxInstance>>),
    Nil,
}
impl Object {
    pub fn is_truphy(&self) -> bool {
        match self {
            Object::Boolean(x) => x.clone(),
            Object::Nil => false,
            _ => true,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Boolean(x), Object::Boolean(y)) => x == y,
            (Object::Number(x), Object::Number(y)) => x == y,
            (Object::String(x), Object::String(y)) => x == y,
            (Object::Nil, Object::Nil) => true,
            (_, _) => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Boolean(x) => write!(f, "{}", x),
            Object::String(x) => write!(f, "{}", x),
            Object::Number(x) => write!(f, "{}", x),
            Object::Call(_) => write!(f, "function"),
            Object::ClassInstance(x) => write!(f, "{}", x.borrow()),
            Object::Nil => write!(f, "nil"),
        }
    }
}
