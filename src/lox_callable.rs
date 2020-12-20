use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::object::Object;
use core::fmt::Debug;
use dyn_clone::DynClone;

pub trait Callable: Debug + DynClone {
    fn arity(&self) -> usize;
    fn call(&self, arguments: &[Object], environment: &mut Interpreter) -> Result<Object>;
}

dyn_clone::clone_trait_object!(Callable);
