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