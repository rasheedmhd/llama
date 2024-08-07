use std::cell::RefCell;
use crate::expr::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}
// var a = 5; print a;  { var a = 7; print a; } print a;

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
    }

    pub fn new_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self { values: HashMap::new(), enclosing }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> EnvResult {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        };

        if self.enclosing.is_some() {
            let enclosing = self.enclosing.clone();
            return Ok(enclosing.unwrap().borrow_mut().get(&name).unwrap());
        }

        return Err(RuntimeError {
            token: name.clone(),
            msg: format!("Oopsie, looks like you forgot to define {} as a variable, (scratches head)", name.lexeme),
        });
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if let Some(entry) = self.values.get_mut(&name.lexeme) {
            *entry = value;
            Ok(())
        } else if self.enclosing.is_some() {
            let enclosing = self.enclosing.clone();
            enclosing.unwrap().borrow_mut().assign(&name, value).unwrap();
            return Ok(())
        } else {
            Err(RuntimeError {
                token: name.clone(),
                msg: format!("Assign here!, Oopsie, looks like you forgot to define '{}' as a variable, (scratches head)", name.lexeme),
            })
        }
    }
}