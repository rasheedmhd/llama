use std::collections::HashMap;
use crate::expr::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;

pub struct Environment {
    pub values: HashMap<String, Literal>
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {

    pub fn new() -> Self { Self { values: HashMap::new() } }
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> EnvResult {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        };

        return Err(RuntimeError {
            token: name.clone(),
            msg: format!("OOpsie, looks like you forgot to define {} as a variable, (scratches head)", name.lexeme),
        });
    }
}