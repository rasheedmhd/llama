// package com.craftinginterpreters.lox;
//
//     class AstPrinter implements Expr.Visitor<String> {
//         String print(Expr expr) {
//             return expr.accept(this);
//         }
//     }

use crate::expr::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor};
struct ast_printer;

impl<String>  Visitor<String> for  ast_printer {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
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