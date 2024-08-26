use std::fmt;
use crate::expr::Literal;
use crate::interpreter::Interpreter;
use crate::runtime_error::RuntimeError;

type LiteralResult = Result<Literal, RuntimeError>;


pub(crate) trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> LiteralResult;
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


// IMPL NATIVE FUNCTIONS
// TO DO
// To add more native functions,
// I will move this into
// function/native_functions.rs
// And also move function.rs into
// functions/function.rs
#[derive(Clone, Debug, PartialEq)]
pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Self
    }

    pub fn arity() -> u8 { 0 }

    pub fn to_string(&self) -> &str {
        "<native function>"
    }
}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Literal>) -> LiteralResult {
        let now = std::time::SystemTime::now();
        let time = now.duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to  properly read Time")
            .as_millis() as f64;

        Ok(Literal::Number(time))
    }

}