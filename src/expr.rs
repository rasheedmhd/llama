#[warn(unused_imports)]

use crate::token::Token;

// To Do
// Type refactor for literal value
// pub enum LiteralValue {
//     Integer(i32),
//     Float(f64),
//     String(String),
//     // Add other variants as needed
// }


pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

// Define the Visitor trait
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
pub trait Visitor<T> {
    fn visit_binary_expr(&mut self,   expr: &BinaryExpr)   -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self,  expr: &LiteralExpr)  -> T;
    fn visit_unary_expr(&mut self,    expr: &UnaryExpr)    -> T;
}


// Implement the accept method for each expression type
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}

pub struct BinaryExpr {
    pub left : Box<Expr>,
    pub operator : Token,
    pub right : Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left : Box<Expr>, operator : Token, right : Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub struct GroupingExpr {
    pub expression : Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression : Box<Expr>) -> Self {
        Self {
            expression,
        }
    }
}

pub struct LiteralExpr {
    pub value : String,
}

impl LiteralExpr {
    pub fn new(value : String) -> Self {
        Self {
            value,
        }
    }
}

pub struct UnaryExpr {
    pub operator : Token,
    pub right : Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator : Token, right : Box<Expr>) -> Self {
        Self {
            operator,
            right,
        }
    }
}

// The data we will visit
mod ast {
    // pub enum Stmt {
    //     Expr(Expr),
    //     Let(Name, Expr),
    // }
    //
    // pub struct Name {
    //     value: String,
    // }
    //
    // pub enum Expr {
    //     IntLit(i64),
    //     Add(Box<Expr>, Box<Expr>),
    //     Sub(Box<Expr>, Box<Expr>),
    // }

}

// The abstract visitor
mod visit {
    // use ast::*;
    use crate::expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};

    // pub trait Visitor<T> {
    //     fn visit_binary_expr(&mut self,   expr: &BinaryExpr)   -> T;
    //     fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    //     fn visit_literal_expr(&mut self,  expr: &LiteralExpr)  -> T;
    //     fn visit_unary_expr(&mut self,    expr: &UnaryExpr)    -> T;
    // }
}

// use ast::*;
// use visit::*;

// An example concrete implementation - walks the AST interpreting it as code.
// struct Interpreter;
// impl Visitor<i64> for Interpreter {
//     fn visit_name(&mut self, n: &Name) -> i64 {
//         panic!()
//     }
//     fn visit_stmt(&mut self, s: &Stmt) -> i64 {
//         match *s {
//             Stmt::Expr(ref e) => self.visit_expr(e),
//             Stmt::Let(..) => unimplemented!(),
//         }
//     }
//
//     fn visit_expr(&mut self, e: &Expr) -> i64 {
//         match *e {
//             Expr::IntLit(n) => n,
//             Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
//             Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
//         }
//     }
// }