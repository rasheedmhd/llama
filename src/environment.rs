use crate::expr::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {
    pub fn new() -> Self {
        let mut env = HashMap::new();
        env.insert("test".to_string(), Literal::String("Empty Env".to_string()));
        Self { values: env }
    }
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> EnvResult {
        self.values
            .get(&name.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError {
                token: name.clone(),
                msg: format!(
                    "Oopsie, looks like you forgot to define '{}' as a variable, (scratches head)",
                    name.lexeme
                ),
            })
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if let Some(entry) = self.values.get_mut(&name.lexeme) {
            *entry = value;
            Ok(())
        } else {
            Err(RuntimeError {
                token: name.clone(),
                msg: format!("Assign here!, Oopsie, looks like you forgot to define '{}' as a variable, (scratches head)", name.lexeme),
            })
        }
    }
}