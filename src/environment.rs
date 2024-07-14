use std::collections::HashMap;
use crate::expr::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;

struct Environment {
    values: HashMap<String, Literal>
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {
    fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    fn get(&mut self, name: Token) -> EnvResult {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        };

        return Err(RuntimeError {
            token: name,
            msg: format!("OOpsie, looks like you forgot to define {} as a variable, (scratches head)", name.lexeme),
        });
    }
}