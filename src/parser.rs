use crate::expr::ast::{BinaryExpr, Expr};
use crate::token::Token;
use crate::token_type::TokenType;


type  ExprBoxed = Box<Expr>;

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

    // expression      → equality
    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Box<Expr> {

        let mut expr = Box::new(self.comparison());

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: ExprBoxed =  Box::new(self.comparison());

            expr = Box::new(Expr::Binary(
               BinaryExpr {
                   left: expr,
                   operator,
                   right,
               }
            ));
        }

        expr
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for r#type in types {
            if self.check(r#type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).clone().unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).clone().unwrap().clone()
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

}