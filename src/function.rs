use crate::callable::Callable;
use crate::environment::Environment;
use crate::expr::Literal;
use crate::interpreter::Interpreter;
use crate::runtime_error::RuntimeError;
use crate::stmt::FunctionStmt;

type LiteralResult = Result<Literal, RuntimeError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    declaration: Box<Option<FunctionStmt>>,
}

impl Function {
    pub(crate) fn _new() -> Self {
        Self {
            declaration: Box::new(None),
        }
    }
    pub(crate) fn from(declaration: Box<Option<FunctionStmt>>) -> Self {
        Self { declaration }
    }
    fn declaration(declaration: Box<Option<FunctionStmt>>) -> Self {
        Self { declaration }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.clone().unwrap().params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> crate::interpreter::LiteralResult {
        let environment = Environment::new();
        // Properly handle the unwrapping
        for (i, param) in self.declaration.clone().unwrap().params.iter().enumerate() {
            interpreter
                .environment
                .define(param.lexeme.clone(), arguments[i].clone());
        }
        interpreter.execute_block(self.declaration.clone().unwrap().body.clone(), environment)?;
        Ok(Literal::Nil)
    }

    fn to_string(&self) -> String {
        format!(
            "<function {}>",
            self.declaration.clone().unwrap().name.lexeme
        )
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
}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Literal>) -> LiteralResult {
        let now = std::time::SystemTime::now();
        let time = now
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to  properly read Time")
            .as_millis() as f64;

        Ok(Literal::Number(time))
    }

    fn to_string(&self) -> String {
        "<native function>".to_string()
    }
}
