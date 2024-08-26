use crate::expr::Literal;
use crate::interpreter::Interpreter;
use crate::runtime_error::RuntimeError;

type LiteralResult = Result<Literal, RuntimeError>;


pub(crate) trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> LiteralResult;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Callee {
    pub arity : usize,
}

impl Callee {
    pub fn new() -> Self {
        Self {
            arity: 0
        }
    }

    pub fn arity() -> u8 { 0 }

    pub fn to_string(&self) -> String {
        "<native function>".to_string()
    }
}

impl Callable for Callee {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Literal>) -> LiteralResult {
        let now = std::time::SystemTime::now();
        let time = now.duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to  properly read Time")
            .as_millis() as f64;

        Ok(Literal::Number(time))
    }

}