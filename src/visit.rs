use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
}
pub trait Accept<T> {
    fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T;
}

impl<T> Accept<T> for Expr {
    fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}
//
// impl<T> Accept<T> for BinaryExpr {
//     fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T {
//         visitor.visit_binary_expr(self)
//     }
// }
// impl<T> Accept<T> for GroupingExpr {
//     fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T {
//         visitor.visit_grouping_expr(self)
//     }
// }
// impl<T> Accept<T> for LiteralExpr {
//     fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T {
//         visitor.visit_literal_expr(self)
//     }
// }
// impl<T> Accept<T> for UnaryExpr {
//     fn accept<V: Visitor<T>>(&self, visitor: &mut V) -> T {
//         visitor.visit_unary_expr(self)
//     }
// }