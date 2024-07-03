use std::any::Any;
use crate::expr::ast::{Expr, BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, LiteralValue};
use crate::visit::{Visitor, Accept};

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: &Box<Expr>) -> Box<dyn Any> {
        expr.accept(self)
    }
}
impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Box<dyn Any> {
        Box::new("Hello")
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Box<dyn Any> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Box<dyn Any> {
        Box::new(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Box<dyn Any> {
        Box::new(11)
    }
}