use crate::environment::Environment;
use crate::expr;
use crate::expr::{
    AssignExpr, BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr, VariableExpr,
};
use crate::repl::Llama;
use crate::runtime_error::RuntimeError;
use crate::stmt;
use crate::stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt};
use crate::token_type::TokenType;
// use std::cell::RefCell;
// use std::rc::Rc;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

// Hack for global variables
lazy_static! {
    static ref ENVIRONMENT: RwLock<HashMap<String, Literal>> = {
        let values = HashMap::new();
        RwLock::new(values)
    };
}

pub struct Interpreter {
    // environment: Environment,
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
        //
        // CHECKPOINT
        // NOTE:
        // Stopped using Environment to use lazy_static
        let literal = self.evaluate(&stmt.initializer)?;
        let mut env = ENVIRONMENT.write().unwrap();
        env.insert(stmt.name.lexeme.clone(), literal);

        // self.environment.define(stmt.name.lexeme.clone(), literal);
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
                        msg: "OOpsie, I was expecting a num but found something else ".to_string(),
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
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::BangEQUAL => return Ok(Literal::Bool(!left.is_equal(&right))),
            TokenType::EqualEQUAL => return Ok(Literal::Bool(left.is_equal(&right))),
            _ => (),
        }

        if expr.operator.token_type == TokenType::PLUS {
            if left.is_string() && right.is_string() {
                let left = left.unwrap_string();
                let right = right.unwrap_string();

                let str_lit = format!("{left}{right}");
                return Ok(Literal::String(str_lit));
            } else if !left.is_num() || !right.is_num() {
                return Err(RuntimeError {
                    token: expr.operator.clone(),
                    msg: "OOpsie, I was expecting two numbers or two strings (scratches head)"
                        .to_string(),
                });
            }
        }

        if !left.is_num() && !right.is_num() {
            return Err(RuntimeError {
                token: expr.operator.clone(),
                msg: "OOpsie, I was expecting two numbers (scratches head)".to_string(),
            });
        }

        let left = left.unwrap_num();
        let right = right.unwrap_num();

        match expr.operator.token_type {
            TokenType::PLUS => return Ok(Literal::Number(left + right)),
            TokenType::MINUS => return Ok(Literal::Number(left - right)),
            TokenType::SLASH => return Ok(Literal::Number(left / right)),
            TokenType::STAR => return Ok(Literal::Number(left * right)),
            TokenType::GREATER => return Ok(Literal::Bool(left > right)),
            TokenType::GreaterEQUAL => return Ok(Literal::Bool(left >= right)),
            TokenType::LESS => return Ok(Literal::Bool(left < right)),
            TokenType::LessEQUAL => return Ok(Literal::Bool(left <= right)),

            // unreachable
            _ => panic!(),
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> LiteralResult {
        // returns a literal of the variable or
        // a runtime error, if variable wasn't declared [properly]
        // let env = ENVIRONMENT.read().unwrap();
        // if let Some(value) = env.get(&expr.name) {
        //     // println!("Value for 'key1': {:?}", value);
        //     Ok(value)
        // }

        Ok(ENVIRONMENT
            .read()
            .unwrap()
            .get(&expr.name.lexeme)
            .unwrap()
            .clone())
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> LiteralResult {
        let value = self.evaluate(&expr.value)?;
        // self.environment.assign(&expr.name, value.clone())?;
        // ENVIRONMENT.write().unwrap().get_mut(&(expr.name.lexeme)).insert(&mut value.clone());
        let mut env = ENVIRONMENT.write().unwrap();
        if let Some(val) = env.get_mut(&expr.name.lexeme) {
            *val = value.clone();
        }
        Ok(value)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            // environment: Environment::new(),
        }
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
