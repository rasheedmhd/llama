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

    fn expression(&self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&self) -> Expr {

        let expr = comparison();

        while self.match_token(TokenType::BangEQUAL, TokenType::EqualEQUAL) {
            let operator = self.previous();
            let right: ExprBoxed = comparison();
            let expr = Expr::Binary(
               BinaryExpr {
                   left: expr,
                   operator,
                   right,
               }
            );
        }

        expr
    }

    fn match_token(&self, types: &[Box<TokenType>]) -> bool {
        for t_type in types {
            if check(t_type) {
                advance();
                return true;
            }
        }
        return false;
    }

    fn check(token_type: TokenType) -> bool {
        if is_at_end() { return flase; }
        return peek().type == toke_type;

    }

    fn advance(&mut self) -> Token {
        if !is_at_end() { self.current += 1; }
        return previous();
    }

    fn is_at_end() -> bool {
        // To Do
        let peek = peek();
        type_of(&peek) == TokenType::EOF;
        peek
    }

    fn peek(&self) -> Token {
        // self.tokens.get(self.current).clone().unwrap().clone()
        self.tokens.get(self.current).unwrap_or_default().clone()
    }

    fn previous(&self) -> Token {
        // self.tokens.get(self.current - 1).clone().unwrap().clone()
        self.tokens.get(self.current).unwrap_or_default().clone()

    }


}