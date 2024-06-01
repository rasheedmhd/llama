use crate::expr::ast::Expr;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(&mut self, tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens,
        }
    }

    fn expression() -> Expr {
        equality()
    }
}