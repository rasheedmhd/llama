use crate::environment::Environment;
use crate::expr::{Callable, Literal};
use crate::interpreter::Interpreter;
use crate::stmt::FunctionStmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub arity: u8,
    declaration: Box<Option<FunctionStmt>>
}

impl Function {
    pub(crate) fn new() -> Self {
        Self {
            arity : 0,
            declaration : Box::new(None),
        }
    }
    pub(crate) fn from(declaration: Box<Option<FunctionStmt>>) -> Self {
        Self {
            arity : 0,
            declaration,
        }
    }
    fn declaration(declaration: Box<Option<FunctionStmt>>) -> Self {
        Self { arity : 0,  declaration }
    }

    pub fn arity(&self) -> u8 { self.declaration.clone().unwrap().params.len() as u8 }

    pub fn to_string(&self) -> String {
        format!("<function {}>", self.declaration.clone().unwrap().name.lexeme)
    }

}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> crate::interpreter::LiteralResult {
        let environment = Environment::new();
        for (i, param) in self.declaration.clone().unwrap().params.iter().enumerate() {
            interpreter.environment.define(param.lexeme.clone(), arguments[i].clone());
        }
        interpreter.execute_block(self.declaration.clone().unwrap().body.clone(), environment)?;
        Ok(Literal::Nil)
    }

}