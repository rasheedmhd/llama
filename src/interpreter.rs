use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::visit::{Accept, Visitor};
use std::any::Any;
use crate::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: &Box<Expr>) -> Box<dyn Any> {
        expr.accept(self)
    }

    fn is_truthy(unary_expr: Box<dyn Any>) -> bool {
        if unary_expr.is::<Option<()>>() {
            return false;
        }
        if let Some(boolean) = unary_expr.downcast_ref::<bool>() {
            return *boolean;
        }
        true
    }
}

impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Box<dyn Any> {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Box<dyn Any> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Box<dyn Any> {
        Box::new(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Box<dyn Any> {
        let right = self.evaluate(&expr.right);

        // borrow the value returned from evaluating the right side of the
        // unary expression which is a Box<dyn Any> borrow it
        // and deference to get the value inside of it. -> an Expr
        // We downcast it into a Concrete type an Expr
        let operand = (&*right).downcast_ref::<f64>();
        if expr.operator.token_type == TokenType::BANG {
            return Box::new(!Self::is_truthy(right));
        } else if expr.operator.token_type == TokenType::MINUS {
            return Box::new(-operand.unwrap());
        };
        // unreachable
        right
    }
}