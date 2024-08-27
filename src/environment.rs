use crate::expr::Literal;
use crate::function::Clock;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Drop for Environment {
    fn drop(&mut self) {
        println!("Dropping Environment!");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    // Serves as the outer block
    // When it is set to None, it means the environment
    // that is created is the last environment alias
    // the global environment
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        let callable = Literal::Function(Rc::new(Clock::new()));
        globals.insert("clock".to_string(), callable);
        Self {
            enclosing: None,
            values: globals,
        }
    }

    pub fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> EnvResult {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        };

        if let Some(enclosing) = &self.enclosing {
            return Ok(enclosing.borrow_mut().get(&name)?);
        }

        return Err(RuntimeError {
            token: name.clone(),
            msg: format!(
                "Oopsie, looks like you forgot to declare {} as a variable",
                name.lexeme
            ),
        });
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if let Some(entry) = self.values.get_mut(&name.lexeme) {
            *entry = value;
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(&name, value)?;
            return Ok(());
        } else {
            Err(RuntimeError {
                token: name.clone(),
                msg: format!("I can't assign a value to an undeclared variable, looks like you forgot to define '{}' as a variable", name.lexeme),
            })
        }
    }
}