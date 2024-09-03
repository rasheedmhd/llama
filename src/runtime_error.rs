use crate::expr::Literal;
use crate::token::Token;
#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub msg: String,
    pub r#return: Option<Literal>
}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> Self {
        Self { token, msg, r#return: None }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // To Do
        // Work on Error message, not all errors are Invalid Syntax errors
        write!(
            f,
            "{} \n\nInvalid Syntax: [ Token '{}' ]  Error at line: {}",
            self.msg, self.token.lexeme, self.token.line
        )
    }
}