use crate::lox_callable::Callable;
use core::fmt::Debug;
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),
    String(String),
    Number(f64),
    Call(Box<dyn Callable>),
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
            (Object::Call(_), _) => false,
            (_, Object::Call(_)) => false,
            (x, y) => x == y,
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
            Object::Nil => write!(f, "nil"),
        }
    }
}
