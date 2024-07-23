use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, Literal, UnaryExpr, VariableExpr, AssignExpr};
use crate::expr;
use crate::repl::Llama;
use crate::stmt;
use crate::token_type::TokenType;
use crate::runtime_error::RuntimeError;
use crate::stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>
}
type LiteralResult = Result<Literal, RuntimeError>;
type StmtResult = Result<(), RuntimeError>;

impl stmt::Visitor<StmtResult> for Interpreter {

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> StmtResult {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> StmtResult {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> StmtResult {
        // To Do
        // CHECKPOINT
        // define works but doesn't persist the changes to environment
        let literal = self.evaluate(&stmt.initializer)?;
        println!("{:?}", self.environment);
        (*self.environment).borrow_mut().define(stmt.name.lexeme.clone(), literal);
        println!("{:?}", self.environment);
        Ok(())
    }
}

impl expr::Visitor<LiteralResult> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralResult {
         Ok(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> LiteralResult {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::BANG => Ok(Literal::Bool(!right.is_truthy())),
            TokenType::MINUS => {
                return if !right.is_num() {
                    Err(RuntimeError {
                        token: expr.operator.clone(),
                        msg: "OOpsie, I was expecting a num but found something else ".to_string()
                    })
                } else {
                    Ok(Literal::Number(-right.unwrap_num()))
                }
            }
            _ => panic!(), //To Do:
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> LiteralResult {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> LiteralResult {
        let left = self.evaluate(&expr.left)?;
        let right= self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::BangEQUAL => return Ok(Literal::Bool(!left.is_equal(&right))),
            TokenType::EqualEQUAL => return Ok(Literal::Bool(left.is_equal(&right))),
            _ => (),
        }

        if expr.operator.token_type  == TokenType::PLUS  {
            if left.is_string() && right.is_string() {

                let left  = left.unwrap_string();
                let right = right.unwrap_string();

                let str_lit = format!("{left}{right}");
                return Ok(Literal::String(str_lit));

            } else if !left.is_num() || !right.is_num() {
                return Err(RuntimeError {
                    token: expr.operator.clone(),
                    msg: "OOpsie, I was expecting two numbers or two strings (scratches head)".to_string()
                })
            }
        }

        if !left.is_num() && !right.is_num() {
            return Err(RuntimeError {
                token: expr.operator.clone(),
                msg: "OOpsie, I was expecting two numbers (scratches head)".to_string()
            })
        }

        let left  = left.unwrap_num();
        let right = right.unwrap_num();

        match expr.operator.token_type {

            TokenType::PLUS => {
                return Ok(Literal::Number(left + right))
            },
            TokenType::MINUS => {
                return Ok(Literal::Number(left - right))
            },
            TokenType::SLASH => {
                return Ok(Literal::Number(left / right))
            },
            TokenType::STAR  => {
                return Ok(Literal::Number(left * right))
            },
            TokenType::GREATER      =>  {
                return Ok(Literal::Bool(left > right))
            },
            TokenType::GreaterEQUAL => {
                return Ok(Literal::Bool(left >= right))
            },
            TokenType::LESS  => {
                return Ok(Literal::Bool(left < right))
            },
            TokenType::LessEQUAL    => {
                return Ok(Literal::Bool(left <= right))
            },

            // unreachable
            _ => panic!()
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> LiteralResult {
        // returns a literal of the variable or
        // a runtime error, if variable wasn't declared [properly]
        (*self.environment).borrow().get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> LiteralResult {
        let value = self.evaluate(&expr.value)?;
        self.environment.borrow_mut().assign(&expr.name, value.clone()).unwrap();
        Ok(value)
    }
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter { environment: Rc::new(RefCell::new(Environment::new())) }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> () {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                Llama::runtime_error(e);
            }
        }
        ()
    }

    fn execute(&mut self, stmt: Stmt) -> StmtResult {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> LiteralResult {
        expr.accept(self)
    }
    // second Impl
    // fn is_truthy(lit: &Literal) -> bool {
    //     // https://doc.rust-lang.org/std/macro.matches.html
    //     // Anything that is not Nil or False is true
    //     !matches!(lit, Literal::Nil | Literal::Bool(false))
    // }
}