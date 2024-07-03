use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::visit::{Accept, Visitor};
use std::any::Any;
use crate::token::Token;
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

    fn is_equal(left: Box<dyn Any>, right: Box<dyn Any> ) -> bool {
        // To Do
        // Keep an Eye on this impl
        if left.is::<Option<()>>() && right.is::<Option<()>>() { return true};
        if left.is::<Option<()>>() { return false };

        if left.is::<f64>() ==  right.is::<f64>(){
            // return Box::new(left_fl.unwrap() + right_fl.unwrap());
            return true;
        };
        if left.is::<String>() &&  right.is::<String>(){
            // return Box::new(left_str.unwrap().clone() + right_str.unwrap());
            return true;
        };
        return false;
        // return Box::new(());
        // let first = first_expr.is::<Option<()>>();
        // let second = second_expr.is::<Option<()>>();
        // if first_expr.is::<Option<()>>() && second_expr.is::<Option<()>>() { return true};
        // if first_expr.is::<Option<()>>() { return false };
        // first_expr == second_expr
        // true
        // match (first, second) {
        //     (None, None) => true,
        //     (None, Some(_)) | (Some(_), None) => false,
        //     (Some(first), Some(second)) => first== second,
        //     _ => true
        // }
    }


    fn check_number_operand(operator: &Token, operand: &Box<dyn Any>) {
        // match operand.is::<f64>() {
        //     f64 => {
        //         return ()
        //     },
        //     Err(_) => {
        //         Err(RuntimeError { token: operator, msg: "OOOps, I was expecting numbers.".to_string() })
        //     }
        // }.expect("TODO: panic message");
        if operand.is::<f64>() { return (); };
        // TO DO
        // Copy Rust Error Ergonomics,
        // Providing error codes to run that explains the Error
        // Err(RuntimeError { token: operator, msg: "OOOps, I was expecting numbers.".to_string()})
        panic!("OOOps, I was expecting numbers.");
    }

    fn check_number_operand_bin(operator: &Token, left: &Box<dyn Any>, right: &Box<dyn Any>) {
        if left.is::<f64>() && right.is::<f64>() { return (); };
        // Copy Rust Error Ergonomics,
        // Providing error codes to run that explains the Error
        // Err(RuntimeError { token: operator, msg: "OOOps, I was expecting numbers.".to_string()})
        panic!("OOOps, I was expecting two numbers.");
    }
}

impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Box<dyn Any> {
        let left = self.evaluate(&expr.left);
        let right= self.evaluate(&expr.right);
        // Using shadowing to convert the left and right parts of the
        // binary expr into concrete values
        let left_fl  = (&*left).downcast_ref::<f64>();
        let right_fl = (&*right).downcast_ref::<f64>();

        let left_str = (&*left).downcast_ref::<String>();
        let right_str = (&*right).downcast_ref::<&str>();

        match expr.operator.token_type {
            TokenType::GREATER      => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() > right_fl.unwrap())
            },
            TokenType::LessEQUAL    => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() <= right_fl.unwrap())
            },
            TokenType::GreaterEQUAL => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() >= right_fl.unwrap())
            },
            TokenType::LESS  => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() < right_fl.unwrap())
            },
            TokenType::MINUS => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() - right_fl.unwrap())
            },
            TokenType::SLASH => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() / right_fl.unwrap())
            },
            TokenType::STAR  => {
                Self::check_number_operand_bin(&expr.operator, &left, &right);
                Box::new(left_fl.unwrap() * right_fl.unwrap())
            },
            TokenType::PLUS  => {
                if left.is::<f64>() &&  right.is::<f64>(){
                    return Box::new(left_fl.unwrap() + right_fl.unwrap());
                };
                if left.is::<String>() &&  right.is::<String>(){
                    return Box::new(left_str.unwrap().clone() + right_str.unwrap());
                };
                panic!("OOOps, I was expecting two numbers or strings.");
                // return Box::new(());
            },
            TokenType::BangEQUAL  => { Box::new(!Self::is_equal(left, right)) },
            TokenType::EqualEQUAL => { Box::new(Self::is_equal(left,  right)) },
            // unreachable
            _ => right
        }
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
        match expr.operator.token_type {
            TokenType::BANG  => { Box::new(!Self::is_truthy(right)) },
            TokenType::MINUS => {
                Self::check_number_operand(&expr.operator, &right);
                Box::new(-operand.unwrap())
            },
            // unreachable
            _ => right
        }
    }
}

struct RuntimeError {
    token: Token,
    msg: String
}

impl RuntimeError {
    fn new(token: Token, msg: String ) -> Self {
       Self {
           token,
           msg: String::new()
       }
    }
}