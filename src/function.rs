use crate::environment::Environment;
use crate::expr::{Callable, Literal};
use crate::interpreter::Interpreter;
use crate::stmt::FunctionStmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    declaration: FunctionStmt
}

impl Function {
    fn declaration(declaration: FunctionStmt) -> Self {
        Self { declaration }
    }

    pub fn arity(&self) -> u8 { self.declaration.params.len() as u8 }

    pub fn to_string(&self) -> String {
        format!("<function {}>", self.declaration.name.lexeme)
    }

}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> crate::interpreter::LiteralResult {
        let environment = Environment::new();
        for (i, param) in self.declaration.params.iter().enumerate() {
            interpreter.environment.define(param.lexeme.clone(), arguments[i].clone());
        }
        interpreter.execute_block(self.declaration.body.clone(), environment)?;
        Ok(Literal::Nil)
    }
}