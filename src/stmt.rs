use crate::expr::Expr;
type BoxedExpr = Box<Expr>;
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
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

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
        }
    }
}