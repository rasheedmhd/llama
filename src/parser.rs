use crate::expr::{
    AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr,
};
use crate::expr::{CallExpr, Literal, LogicalExpr};
use crate::repl::Llama;
use crate::stmt::{
    BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, VarStmt,
    WhileStmt,
};
use crate::token::Token;
use crate::token_type::TokenType;
use std::fmt;

type BoxedExpr = Box<Expr>;
type StmtResult = Result<Stmt, ParseError>;
pub struct ParseError;

impl ParseError {
    pub fn new() -> Self {
        Self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

type ExprResult = Result<BoxedExpr, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// THIS IS THE GRAMMAR THE PARSER DESCENDS ON
// program         → declaration* EOF ;
// declaration 	   → funDecl | varDecl | statement ;
// funDecl         → "fun" function ;
// function        → IDENTIFIER "(" parameters? ")" block ;
// parameters         → IDENTIFIER ( " , " IDENTIFIER )* ;
// varDecl         → "var" IDENTIFIER ( "=" expression )? ";" ;

// statement 	   → exprStmt | forStmt | ifSmt | printStmt | whileStmt | block ;
// forStmt         → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";"  expression? ")"  statement ;
// exprStmt        → expression ";" ;
// printStmt       → "print" expression ";" ;
// whileStmt       → "while" "(" expression ")" statement ;
// block           → "{" declaration* "}"
// ifStmt          → "if" "(" expression ")" statement ( "else" statement )? ;

// expression      → assigment ;
// assignment 	   → IDENTIFIER "=" assignment | logic_or ;
// logic_or        → logic_and ( "or" logic_and )* ;
// logic_and       → equality ( "and" equality )* ;
// equality        → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison      → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term            → factor ( ( "-" | "+" ) factor )* ;
// factor          → unary ( ( "/" | "*" ) unary )* ;
// unary           → ( "!" | "-" ) unary | primary ;
// call            → primary ( "(" arguments? ")" )* ;
// arguments       → expression ( "," expression )* ;
// primary         → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn from(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            // Get a variable declaration statements and add to
            // statements above, continue doing that till the end
            // of the code / source code file
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    // declaration  → funDecl | varDecl | statement ;
    fn declaration(&mut self) -> StmtResult {
        // If we hit a var token, then we are dealing with a variable expr
        // So we pass control to var_declaration() to parse it
        // Else executing falls through to statement() to parse
        if self.match_token(&[TokenType::FUN]) {
            return self.function("function");
        } else if self.match_token(&[TokenType::VAR]) {
            return self.var_declaration();
        }

        // What did statement() return ?
        // An error ? Then synchronize and return a None/Nil Literal
        // wrapped in an Expression
        // Else return the whole statement result to the function
        // calling declaration() to handle
        // Essentially here is where we handle Errors
        let stmt_result = self.statement();
        match stmt_result {
            Err(_) => {
                self.synchronize();
                // construct expr
                let expr = Box::new(Expr::Literal(LiteralExpr {
                    value: Literal::Nil,
                }));
                // wrap constructed expr above into a statement expression
                let stmt_expr = Stmt::Expression(ExpressionStmt { expression: expr });
                // return the statement expression after synchronizing above
                // To Do
                // I think this is easier to read but might have perf hit,
                // throw this into godbolt.org and investigate
                return Ok(stmt_expr);
            }
            _ => stmt_result,
        }
    }

    // varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> StmtResult {
        let name = self.consume(
            &TokenType::IDENTIFIER,
            "Aww snap! [*_*], looks like you forgot to add a name for the variable",
        )?;
        let mut initializer = Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        }));
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = self.expression().unwrap();
        };
        self.consume(
            &TokenType::SEMICOLON,
            "I was expecting a ';' after the variable declaration, (scratches head).",
        )?;
        let var_statement = VarStmt { name, initializer };
        Ok(Stmt::Var(var_statement))
    }

    // statement → exprStmt | IfStmt printStmt | block ;
    fn statement(&mut self) -> StmtResult {
        if self.match_token(&[TokenType::FOR]) {
            return self.for_statement();
        } else if self.match_token(&[TokenType::IF]) {
            return self.if_statement();
        } else if self.match_token(&[TokenType::PRINT]) {
            return Ok(self.print_statement()?);
        } else if self.match_token(&[TokenType::RETURN]) {
            return Ok(self.return_statement()?);
        } else if self.match_token(&[TokenType::WHILE]) {
            return Ok(self.while_statement()?);
        } else if self.match_token(&[TokenType::LeftBRACE]) {
            let block_stmts = self.block()?;
            return Ok(Stmt::Block(BlockStmt {
                statements: block_stmts,
            }));
        };
        return Ok(self.expression_statement()?);
    }

    // expression → assigment ;
    fn expression_statement(&mut self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value")?;
        let expr = ExpressionStmt { expression: expr };
        Ok(Stmt::Expression(expr))
    }

    fn function(&mut self, kind: &str) -> StmtResult {
        let name: Token = self.consume(&TokenType::IDENTIFIER, &format!("Expect {} name", kind))?;
        self.consume(&TokenType::LeftPAREN, &format!("Expect {} name", kind))?;
        let mut params: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RightPAREN) {
            loop {
                if params.len() > 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments");
                }
                params.push(
                    self.consume(&TokenType::IDENTIFIER, "I was expecting a parameter name")?,
                );
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightPAREN, &format!("Expect {} name", kind))?;
        self.consume(
            &TokenType::LeftBRACE,
            "I was expecting a '{' after the arguments",
        )?;
        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt { name, params, body }))
    }

    // forStmt → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
    fn for_statement(&mut self) -> StmtResult {
        self.consume(&TokenType::LeftPAREN, "Expect '(' after 'for'")?;

        let initializer = if self.match_token(&[TokenType::SEMICOLON]) {
            None
        } else if self.match_token(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition = Some(Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Nil,
        })));

        if !self.check(&TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }
        self.consume(&TokenType::SEMICOLON, "Expect ';' after loop condition")?;

        let mut increment = None;
        if !self.check(&TokenType::RightPAREN) {
            increment = Some(self.expression()?);
        };

        self.consume(&TokenType::RightPAREN, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;

        if increment != None {
            body = Stmt::Block(BlockStmt {
                statements: vec![
                    body,
                    Stmt::Expression(ExpressionStmt {
                        expression: increment.unwrap(),
                    }),
                ],
            });
        }

        if condition.is_none() {
            condition = Some(Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Bool(true),
            })));
        }

        if let Some(initializer) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![initializer, body],
            });
        }

        Ok(Stmt::While(WhileStmt {
            condition: condition.unwrap(),
            body: Box::new(body),
        }))
    }

    // ifStmt → "if" "(" expression ")" statement ( "else" statement )? ;
    fn if_statement(&mut self) -> StmtResult {
        self.consume(&TokenType::LeftPAREN, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightPAREN, "Expect ')' after if condition")?;
        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_token(&[TokenType::ELSE]) {
            else_branch = Some(Box::new(self.statement()?))
        }
        let if_stmt = IfStmt {
            condition,
            then_branch,
            else_branch,
        };
        println!("{:#?}", if_stmt);
        return Ok(Stmt::If(if_stmt));
    }

    // printStmt → "print" expression ";" ;
    fn print_statement(&mut self) -> StmtResult {
        let value = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        let value = PrintStmt { expression: value };
        Ok(Stmt::Print(value))
    }

    fn return_statement(&mut self) -> StmtResult {
        let keyword = self.previous();
        let value: Option<Expr> = if !self.check(&TokenType::SEMICOLON) {
            Some(*self.expression()?)
        } else {
            None
        };
        self.consume(
            &TokenType::SEMICOLON,
            "I was expecting a ';' after value to conclude and parse a valid return statement",
        )?;
        let value = ReturnStmt {
            keyword,
            value: Some(Box::new(value.unwrap())),
        };
        Ok(Stmt::Return(value))
    }

    fn while_statement(&mut self) -> StmtResult {
        self.consume(&TokenType::LeftPAREN, "Expect '(' after 'while")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightPAREN, "Expect ')' after condition")?;
        let body = Box::new(self.statement()?);
        let while_stmt = Stmt::While(WhileStmt { condition, body });
        println!("{:#?}", while_stmt);
        Ok(while_stmt)
    }

    //  block → "{" declaration* "}" ;
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        self.consume(&TokenType::RightBRACE,
                     "I was expecting a '}' to close the most current block that you created\n \
                      when you typed '{' after block. \n You might need to add a '}' to create a valid Llama block"
        )?;
        Ok(statements)
    }
    // assignment → IDENTIFIER "=" assignment | logic_or ;
    fn assignment(&mut self) -> ExprResult {
        let expr = self.or()?;

        if self.match_token(&[TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(var_expr) = *expr {
                return Ok(Box::new(Expr::Assign(AssignExpr {
                    name: var_expr.name,
                    value,
                })));
            }

            // To Do
            // Impl delightful error msgs
            Llama::error(equals, "Invalid argument target");
        }

        Ok(expr)
    }

    // logic_or → logic_and ( "or" logic_and )* ;
    fn or(&mut self) -> ExprResult {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::OR]) {
            let operator = self.previous();
            let right: BoxedExpr = self.and()?;

            expr = Box::new(Expr::Logical(LogicalExpr {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    // logic_and → equality ( "and" equality )* ;
    fn and(&mut self) -> ExprResult {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::AND]) {
            let operator = self.previous();
            let right: BoxedExpr = self.equality()?;

            expr = Box::new(Expr::Logical(LogicalExpr {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    // expression → assignment
    fn expression(&mut self) -> ExprResult {
        self.assignment()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEQUAL, TokenType::EqualEQUAL]) {
            let operator = self.previous();
            let right: BoxedExpr = self.comparison()?;

            expr = Box::new(Expr::Binary(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
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

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GreaterEQUAL,
            TokenType::LESS,
            TokenType::LessEQUAL,
        ]) {
            let operator = self.previous();
            let right: BoxedExpr = self.term()?;

            expr = Box::new(Expr::Binary(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    // term  → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right: BoxedExpr = self.factor()?;

            expr = Box::new(Expr::Binary(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    // factor  → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ExprResult {
        let mut expr = BoxedExpr::from(self.unary()?);

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right: BoxedExpr = self.unary()?;

            expr = Box::new(Expr::Binary(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> ExprResult {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            let expr = Box::new(Expr::Unary(UnaryExpr { operator, right }));

            return Ok(expr);
        };
        self.call()
    }

    fn call(&mut self) -> ExprResult {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftPAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: BoxedExpr) -> ExprResult {
        let mut arguments: Vec<BoxedExpr> = Vec::new();

        if !self.check(&TokenType::RightPAREN) {
            loop {
                if arguments.len() > 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments");
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightPAREN, "Expect ')' after arguments")?;
        Ok(Box::new(Expr::Call(CallExpr {
            callee,
            paren,
            arguments,
        })))
    }

    // primary  → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> ExprResult {
        if self.match_token(&[TokenType::FALSE]) {
            let expr = Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Bool(false),
            }));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::TRUE]) {
            let expr = Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Bool(true),
            }));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::NIL]) {
            let expr = Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            }));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let expr = Box::new(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
            return Ok(expr);
        }

        if self.match_token(&[TokenType::IDENTIFIER]) {
            let expr = Box::new(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
            return Ok(expr);
        }

        // The interesting branch is the one for handling parentheses.
        // After we match an opening ( and parse the expression inside it, we must find a ) token.
        // If we don’t, that’s an error.
        // Impl on day break
        if self.match_token(&[TokenType::LeftPAREN]) {
            let mut expr = self.expression()?;
            self.consume(&TokenType::RightPAREN, "Expect ')' after expression.")?;
            expr = Box::new(Expr::Grouping(GroupingExpr { expression: expr }));
            return Ok(expr);
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
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn consume(&mut self, r#type: &TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance());
        };
        Err(self.error(self.peek(), message))
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        Llama::error(token, message);
        ParseError::new()
    }

    // TO DO
    // I think there is an issue here
    // Llama confuses when it synchronises
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}
