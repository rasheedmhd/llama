use crate::expr::Expr;
use crate::token::Token;

type BoxedExpr = Box<Expr>;
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

pub struct ExpressionStmt {
    pub expression : BoxedExpr,
}

impl ExpressionStmt {
    pub fn new(expression : BoxedExpr) -> Self {
        Self {
            expression,
        }
    }
}


pub struct PrintStmt {
    pub expression : BoxedExpr,
}

impl PrintStmt {
    pub fn new(expression : BoxedExpr) -> Self {
        Self {
            expression,
        }
    }
}

pub struct VarStmt {
    pub name : Token,
    pub initializer : BoxedExpr,
}

impl VarStmt {
    pub fn new(name : Token, initializer : BoxedExpr) -> Self {
        Self {
            name,
            initializer,
        }
    }
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
        }
    }
}