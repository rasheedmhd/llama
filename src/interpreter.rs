#[allow(unused_variables)]

use crate::expr::ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, Literal, UnaryExpr};
use crate::visit::Visitor;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::runtime_error::RuntimeError;


pub struct Interpreter;
type LiteralResult = Result<Literal, RuntimeError>;

impl Visitor<LiteralResult> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralResult {
         Ok(expr.value.clone())
    }
    #[allow(unused_variables)]
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> LiteralResult {
        let right = self.evaluate(&expr.right)?;
        //
        // match expr.operator.token_type {
        //     TokenType::MINUS => {
        //         match right {
        //             Literal::Number(right) => return Ok(Literal::Number(-right+11f64)),
        //             _ => Ok(Literal::Nil),
        //         // return Ok(Literal::wrap_num(-right.unwrap_num()));
        //
        //         }
        //     },
        //     _ => Ok(Literal::Nil),
        // }
        match expr.operator.token_type {
            TokenType::MINUS => {
                if !right.is_num() {
                    // number_error(&expr.operator)
                    println!("Expecting number but found something else");
                    Ok(right)
                } else {
                    return Ok(Literal::wrap_num(-right.unwrap_num()));
                }
            }
            TokenType::BANG => Ok(right),
            _ => panic!(), //TODO:
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> LiteralResult {
        self.evaluate(&expr.expression)
    }
    #[allow(unused_variables)]
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> LiteralResult {
        let left = self.evaluate(&expr.left);
        let right= self.evaluate(&expr.right);
        right
        // Using shadowing to convert the left and right parts of the
        // binary expr into concrete values
        // println!("{:?}", right.type_id());
        // let left_fl  = (&*left).downcast_ref::<f64>().unwrap().clone();
        // let right_fl = (&*right).downcast_ref::<f64>().unwrap().clone();
        // println!("{:?}", right_fl.type_id());
        //
        // let left_str = (&*left).downcast_ref::<String>().unwrap().clone();
        // let right_str = (&*right).downcast_ref::<&str>().unwrap().clone();

        // match expr.operator.token_type {
        //     TokenType::GREATER      => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl > right_fl)
        //     },
        //     TokenType::LessEQUAL    => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl <= right_fl)
        //     },
        //     TokenType::GreaterEQUAL => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl >= right_fl)
        //     },
        //     TokenType::LESS  => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl < right_fl)
        //     },
        //     TokenType::MINUS => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl - right_fl)
        //     },
        //     TokenType::SLASH => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl / right_fl)
        //     },
        //     TokenType::STAR  => {
        //         Self::check_number_operand_bin(&expr.operator, &left, &right);
        //         Box::new(left_fl * right_fl)
        //     },
        //     TokenType::PLUS  => {
        //         if left.is::<f64>() &&  right.is::<f64>(){
        //             return Box::new(left_fl + right_fl);
        //         };
        //         if left.is::<String>() &&  right.is::<String>(){
        //             return Box::new(left_str.clone() + right_str);
        //         };
        //         // panic!("OOOps, I was expecting two numbers or strings.");
        //         return Box::new(
        //             RuntimeError { token: expr.operator.clone(), msg: "OOOps, I was expecting numbers.".to_string()}
        //         );
        //     },
        //     TokenType::BangEQUAL  => { Box::new(!Self::is_equal(left, right)) },
        //     TokenType::EqualEQUAL => { Box::new(Self::is_equal(left,  right)) },
        //     // unreachable
        //     _ => right
        // }
    }
}

impl Interpreter {

    pub fn new() -> Self { Interpreter }

    pub fn interpret(&mut self, expr: &Box<Expr>) {
        let lit = self.evaluate(expr)
            .expect("Failed to interpret expression");
        println!("{}", lit);
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> LiteralResult {
        expr.accept(self)
    }
    #[allow(unused_variables)]
    fn is_truthy(unary_expr: LiteralResult) -> bool {
        todo!()
    //     if unary_expr.is::<Option<()>>() {
    //         return false;
    //     }
    //     if let Some(boolean) = unary_expr.downcast_ref::<bool>() {
    //         return *boolean;
    //     }
    //     true
    }
    #[allow(unused_variables)]
    fn is_equal(left: LiteralResult, right: LiteralResult ) -> bool {
        todo!()
    //     // To Do
    //     // Keep an Eye on this impl
    //     if left.is::<Option<()>>() && right.is::<Option<()>>() { return true };
    //     if left.is::<Option<()>>() { return false };
    //
    //     if left.is::<f64>() ==  right.is::<f64>(){
    //         // return Box::new(left_fl.unwrap() + right_fl.unwrap());
    //         return true;
    //     };
    //     if left.is::<String>() &&  right.is::<String>(){
    //         // return Box::new(left_str.unwrap().clone() + right_str.unwrap());
    //         return true;
    //     };
    //     return false;
    //
    //     // match (first, second) {
    //     //     (None, None) => true,
    //     //     (None, Some(_)) | (Some(_), None) => false,
    //     //     (Some(first), Some(second)) => first== second,
    //     //     _ => true
    //     // }
    }
    #[allow(unused_variables)]
    fn check_number_operand(operator: &Token, operand: &LiteralResult) -> Result<(), RuntimeError> {
    todo!()//     if operand.is::<f64>() { return Ok(()); };
    //     // TO DO
    //     // Copy Rust Error Ergonomics, Providing error codes to run that explains the Error
    //     Err(RuntimeError { token: operator.clone(), msg: "OOOps, I was expecting numbers.".to_string()})
    }
    //
    #[allow(unused_variables)]
    fn check_number_operand_bin(operator: &Token, left: &LiteralResult, right: &LiteralResult) -> Result<(), RuntimeError> {
    todo!()
        //     if left.is::<f64>() && right.is::<f64>() { return Ok(()); };
    //     Err(RuntimeError { token: operator.clone(), msg: "OOOps, I was expecting numbers.".to_string()})
    }
}