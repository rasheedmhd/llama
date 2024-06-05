use crate::expr::ast::{BinaryExpr, Expr, UnaryExpr};
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

        let mut expr = self.comparison();

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: ExprBoxed =  self.comparison();

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

    // comparison      → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Box<Expr> {

        let mut expr = self.term();

        while self.match_token(&[TokenType::GREATER, TokenType::GreaterEQUAL, TokenType::LESS, TokenType::LessEQUAL]) {
            let operator = self.previous();
            let right: ExprBoxed =  self.term();

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

    // term            → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<Expr> {

        let mut expr = self.factor();

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: ExprBoxed =  self.factor();

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

    // factor          → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Box<Expr> {

        let mut expr = self.unary();

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: ExprBoxed =  self.unary();

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

    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right: ExprBoxed =  self.unary();
            let expr = Box::new(Expr::Unary(
                UnaryExpr {
                    operator,
                    right,
                }
            ));

            return expr;
        };
        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        todo!()
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

}