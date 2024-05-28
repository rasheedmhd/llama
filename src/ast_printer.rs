
use crate::expr::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, ASTVisitor};
struct ast_printer;

impl<String>  ASTVisitor<String> for  ast_printer {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        // return expr.new();
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        todo!()
    }
}