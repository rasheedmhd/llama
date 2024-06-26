use std::fmt;
use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::expr::ast::LiteralValue;
use crate::repl::Llama;
use crate::token::Token;
use crate::token_type::TokenType;

type  BoxedExpr = Box<Expr>;
pub struct ParseError;

impl ParseError {
    pub fn new() -> Self { Self }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
// THIS IS THE GRAMMAR
// THE PARSER WORKS ON
// expression      → equality ;
// equality        → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison      → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term            → factor ( ( "-" | "+" ) factor )* ;
// factor          → unary ( ( "/" | "*" ) unary )* ;
// unary           → ( "!" | "-" ) unary | primary ;
// primary         → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens,
        }
    }

    // expression      → equality
    fn expression(&mut self) -> BoxedExpr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> BoxedExpr {

        let mut expr = self.comparison();

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.comparison();

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

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
        // To Do, Use Less Cloning and Borrow More = Idiomatic Rust
        // &self.tokens[self.current - 1]
    }

    // comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> BoxedExpr {

        let mut expr = self.term();

        while self.match_token(&[TokenType::GREATER, TokenType::GreaterEQUAL, TokenType::LESS, TokenType::LessEQUAL]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.term();

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

    // term  → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> BoxedExpr {

        let mut expr = self.factor();

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.factor();

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

    // factor  → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> BoxedExpr {

        let mut expr = self.unary();

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.unary();

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
    fn unary(&mut self) -> BoxedExpr {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.unary();
            let expr = Box::new(Expr::Unary(
                UnaryExpr {
                    operator,
                    right,
                }
            ));

            return expr;
        };
        // To Do
        // Propagate error up all the way to expression()
        // unary will return a Result<BoxedExpr, ParseError>
        self.primary().expect("No valid expression found")
    }

    // primary  → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<BoxedExpr, ParseError> {

        if self.match_token(&[TokenType::FALSE]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: LiteralValue::Bool(false)
                }
            ));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::TRUE]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: LiteralValue::Bool(true)
                }
            ));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::NIL]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: LiteralValue::Nil
                }
            ));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: self.previous().literal
                }
            ));
            return Ok(expr);
        }

        // The interesting branch is the one for handling parentheses.
        // After we match an opening ( and parse the expression inside it, we must find a ) token.
        // If we don’t, that’s an error.
        // Impl on day break
        if self.match_token(&[TokenType::LeftPAREN]) {
            let mut expr = self.expression();
            self.consume(&TokenType::RightPAREN, "Expect ')' after expression.").unwrap();
            expr = Box::new(Expr::Grouping(
                GroupingExpr {
                    expression: expr
                }
            ));
            return  Ok(expr);
        }
        Err(self.error(self.peek(), "Expect expression"))
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

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        return self.previous();
    }

    fn consume(&mut self, r#type: &TokenType, message: &str ) -> Result<Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance())
        };
        Err(self.error(self.peek(), message))
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        Llama::error(
            token,
            message
        );
        ParseError::new()
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS |
                TokenType::FUN   |
                TokenType::VAR   |
                TokenType::FOR   |
                TokenType::IF    |
                TokenType::WHILE |
                TokenType::PRINT |
                TokenType::RETURN => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.expression()
        // match self.expression() {
        //     Ok(expr) => Some(expr),
        //     Err(_) => None,
        // }
    }




}