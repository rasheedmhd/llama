use crate::stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt};
use crate::expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::expr::Literal;
use crate::repl::Llama;
use crate::token::Token;
use crate::token_type::TokenType;
use std::fmt;

type  BoxedExpr = Box<Expr>;
type StmtResult = Result<Stmt, ParseError>;
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

type ExprResult = Result<BoxedExpr, ParseError>;

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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
        // match self.expression() {
        //     Ok(expr) => Ok(expr),
        //     Err(_) => Err(ParseError),
        // }
    }

    fn declaration(&mut self) -> StmtResult {
        // If we hit a var token, then we are dealing with a variable expr
        // So we pass control to var_declaration() to parse it
        // Else executing falls through to statement() to parse
        if self.match_token(&[TokenType::VAR]) {
            return self.var_declaration()
        }

        // What did statement() return ?
        // An error ? Then synchronize and return a None/Nil Statement
        // Else return the whole statement result to the function
        // calling declaration() to handle
        /// Essentially here is where we handle Errors
        let stmt_result = self.statement();
        match stmt_result {
            Err(_) => {
                self.synchronize();
                // construct expr
                let expr = Box::new(Expr::Literal ( LiteralExpr { value: Literal::Nil }));
                // wrap constructed expr above into a statement expression
                let stmt_expr = Stmt::Expression( ExpressionStmt { expression: expr });
                // return the statement expression after synchronizing above
                // To Do
                // I think this is easier to read but might have perf hit,
                // throw this into godbolt.org and investigate
                return Ok( stmt_expr );
            }
            _ => stmt_result
        }
    }

    fn var_declaration(&mut self) -> StmtResult {
        let name = self.consume(&TokenType::IDENTIFIER, "Expect variable name")?;
        let mut initializer = Box::new(Expr::Literal ( LiteralExpr { value: Literal::Nil }));
        if self.match_token(&[TokenType::EQUAL]) {
           initializer = self.expression()?;
        };
        self.consume(&TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;
        let var_statement = VarStmt { name, initializer };
        Ok(Stmt::Var(var_statement))
    }

    fn statement(&mut self) -> StmtResult {
        if self.match_token(&[TokenType::PRINT]) {
            return Ok(self.print_statement());
        };
        return Ok(self.expression_statement());
    }
    fn print_statement(&mut self) -> Stmt {
        let value = self.expression().unwrap();
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.").unwrap();
        // shadowing value to pass to Stmt::Print,
        // I believe it helps in reading
        let value = PrintStmt { expression: value };
        Stmt::Print(value)
    }
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression().unwrap();
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.").unwrap();
        let expr = ExpressionStmt { expression: expr };
        Stmt::Expression(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        // let mut expr = self.equality()?;
        //
        // if self.match_token(&[TokenType::EQUAL]) {
        //     let equals = self.previous();
        //     let value = self.assignment()?;
        //
        //     if let Expr::Variable(name) = expr {
        //         return Ok(Expr::Assign( AssignExpr { name, Box::new(value)} ));
        //     }
        //
        //     // self.error(equals, "Invalid assignment target.");
        //     Llama::error(equals, "Invalid argument target");
        // }
        //
        // Ok(expr)]
        todo!()
    }


    // fn assignment(&mut self) -> ExprResult {
    //
    //     let mut expr = self.comparison()?;
    //     if self.match_token(&[TokenType::EQUAL]) {
    //         let equals = self.previous();
    //         let value = self.assignment()?;
    //         match var {
    //             VariableExpr => {
    //
    //             }
    //             let name =
    //             return Expr::Assign(AssignExpr { name, value });
    //         }
    //     Llama::error(equals, "Invalid argument target");
    //     }
    //     Ok(expr)
    // }
    // expression      → equality
    fn expression(&mut self) -> ExprResult {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ExprResult {

        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.comparison()?;

            expr = Box::new(Expr::Binary(
               BinaryExpr {
                   left: expr,
                   operator,
                   right,
               }
            ));
        }
        Ok(expr)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
        // To Do, Use Less Cloning and Borrow More = Idiomatic Rust
        // &self.tokens[self.current - 1]
    }

    // comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> ExprResult {

        let mut expr = self.term()?;

        while self.match_token(&[TokenType::GREATER, TokenType::GreaterEQUAL, TokenType::LESS, TokenType::LessEQUAL]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.term()?;

            expr = Box::new(Expr::Binary(
               BinaryExpr {
                   left: expr,
                   operator,
                   right,
               }
            ));
        }
        Ok(expr)
    }

    // term  → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ExprResult {

        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.factor()?;

            expr = Box::new(Expr::Binary(
                BinaryExpr {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    // factor  → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ExprResult {

        let mut expr = BoxedExpr::from(self.unary()?);

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: BoxedExpr =  self.unary()?;

            expr = Box::new(Expr::Binary(
                BinaryExpr {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> ExprResult {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right=  self.unary()?;
            let expr = Box::new(Expr::Unary(
                UnaryExpr {
                    operator,
                    right,
                }
            ));

            return Ok(expr);
        };
        self.primary()
    }

    // primary  → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> ExprResult {

        if self.match_token(&[TokenType::FALSE]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: Literal::Bool(false)
                }
            ));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::TRUE]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: Literal::Bool(true)
                }
            ));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::NIL]) {
            let expr = Box::new(Expr::Literal(
                LiteralExpr {
                    value: Literal::Nil
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

        if self.match_token(&[TokenType::IDENTIFIER]) {
            let expr = Box::new(Expr::Variable(
                VariableExpr {
                    name: self.previous(),
                }
            ));
            return Ok(expr);
        }

        // The interesting branch is the one for handling parentheses.
        // After we match an opening ( and parse the expression inside it, we must find a ) token.
        // If we don’t, that’s an error.
        // Impl on day break
        if self.match_token(&[TokenType::LeftPAREN]) {
            let mut expr = self.expression()?;
            self.consume(&TokenType::RightPAREN, "Expect ')' after expression.")?;
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

}