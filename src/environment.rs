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
    // enclosing Serves as the outer block
    // When it is set to None, it means the environment
    // that is created is the last environment alias
    // the global environment
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

type EnvResult = Result<Literal, RuntimeError>;

impl Environment {
    // The new constructor is for the global scope’s environment,
    // which ends the chain.
    // The parent: None, shows that it is not enclosed in any outer Environment
    // it is the last/global/ultimate environment
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        let callable = Literal::Function(Rc::new(Clock::new()));
        // Adding Native Functions
        // We only have one called clock to tell time between
        // two successive periods of code execution
        globals.insert("clock".to_string(), callable);
        Self {
            parent: None,
            values: globals,
        }
    }

    // The from constructor creates a new local scope
    // nested inside the given outer one.
    // new and from? Yeah, why not! following Rust's
    // code convention, Head it is called the Factory Pattern
    pub fn from(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> EnvResult {
        // if self.values.contains_key(&name.lexeme) {
        //     return Ok(self.values.get(&name.lexeme).unwrap().clone());
        // };

        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(parent) = &self.parent {
            return Ok(parent.borrow_mut().get(&name)?);
        }

        return Err(RuntimeError {
            token: name.clone(),
            msg: format!(
                "Oopsie, looks like you forgot to declare {} as a variable",
                name.lexeme
            ),
            r#return: None,
        });
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if let Some(entry) = self.values.get_mut(&name.lexeme) {
            *entry = value;
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(&name, value)?;
            return Ok(());
        } else {
            Err(RuntimeError {
                token: name.clone(),
                msg: format!("I can't assign a value to an undeclared variable, looks like you forgot to define '{}' as a variable", name.lexeme),
                r#return: None,
            })
        }
    }
}
