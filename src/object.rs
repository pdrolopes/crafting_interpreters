use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Boolean(bool),
    String(String),
    Number(f64),
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

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Boolean(x) => write!(f, "{}", x),
            Object::String(x) => write!(f, "{}", x),
            Object::Number(x) => write!(f, "{}", x),
            Object::Nil => write!(f, "nil"),
        }
    }
}
