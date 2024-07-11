use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};

pub trait Visitor<T> {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),

        }
    }
}