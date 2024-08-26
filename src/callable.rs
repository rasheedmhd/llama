use std::fmt;
use crate::expr::Literal;
use crate::interpreter::Interpreter;
use crate::runtime_error::RuntimeError;

type LiteralResult = Result<Literal, RuntimeError>;


pub(crate) trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> LiteralResult;
    fn to_string(&self) -> String;
}

impl fmt::Debug for dyn Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callable Function")
    }
}

impl PartialEq for dyn Callable {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}