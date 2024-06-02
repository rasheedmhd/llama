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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {

        let mut expr = comparison();

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: ExprBoxed = comparison();
            expr = Expr::Binary(
               BinaryExpr {
                   left: expr,
                   operator,
                   right,
               }
            );
        }

        expr
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t_type in types {
            if self.check(t_type.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        // To Do
        // return peek().type == toke_type;

    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        // To Do
        // let peek = peek();
        // type_of(&peek) == TokenType::EOF;
        // peek
        true
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).clone().unwrap().clone()
        // self.tokens.get(self.current).unwrap_or_default().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).clone().unwrap().clone()
        // self.tokens.get(self.current).unwrap_or_default().clone()

    }


}