use crate::expr::ast::Literal;
use crate::token::Token;
#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub msg: String
}

impl RuntimeError {
    fn new(token: Token, msg: String ) -> Self {
        Self { token, msg }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} \nInvalid Syntax: [Token {}]", self.msg, self.token.lexeme)
    }
}