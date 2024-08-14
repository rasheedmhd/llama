use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::expr;
use crate::expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, LogicalExpr, UnaryExpr, VariableExpr};
use crate::repl::{Llama, Repl};
use crate::runtime_error::RuntimeError;
use crate::stmt;
use crate::stmt::{BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt};
use crate::token_type::TokenType;

pub struct Interpreter {
    environment: Environment,
}

type LiteralResult = Result<Literal, RuntimeError>;
type StmtResult = Result<(), RuntimeError>;

impl stmt::Visitor<StmtResult> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> StmtResult {
        let value = self.evaluate(&stmt.expression)?;
        // TO DO
        // Prints assignments if you pass a text file, not what I wanted.
        // println!("{value}");
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> StmtResult {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> StmtResult {
        let literal = self.evaluate(&stmt.initializer)?;
        self.environment.define(stmt.name.lexeme.clone(), literal);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> StmtResult {
        let block_env = Environment::new_enclosing(Rc::new(RefCell::new(self.environment.clone())));
        self.execute_block(stmt.statements.clone(), block_env)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> StmtResult {
        let cond = self.evaluate(&stmt.condition)?;
        if cond.is_truthy() {
            self.execute(stmt.then_branch.as_ref().clone())?.clone();
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(*else_branch.clone())?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> StmtResult {
        let cond = self.evaluate(&stmt.condition)?;
        while cond.is_truthy() {
            self.execute(*stmt.body.clone())?
        }
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
        self.environment.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> LiteralResult {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> LiteralResult {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.token_type == TokenType::OR {
            if left.is_truthy() { return Ok(left); }
        } else {
            if !left.is_truthy() { return Ok(left); }
        }
        self.evaluate(&expr.right)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
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

    pub fn execute_block(&mut self, statements: Vec<Stmt>, block_env: Environment) -> Result<(), RuntimeError> {
        let previous_env = std::mem::replace(&mut self.environment, block_env);

        let result = (|| {
            for statement in statements {
                self.execute(statement)?;
                // println!("block_env {:?}", block_env);
                // println!("previous_env {:?}", previous_env);
            }
            Ok(())
        })();

        self.environment = previous_env;

        result
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> LiteralResult {
        expr.accept(self)
    }
}