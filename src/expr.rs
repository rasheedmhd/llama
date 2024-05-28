// To Do
// Type refactor for literal value
// pub enum LiteralValue {
//     Integer(i32),
//     Float(f64),
//     String(String),
//     // Add other variants as needed
// }


pub mod ast {

    type ExprBoxed = Box<Expr>;
    use crate::token::Token;

    pub enum Expr {
        Binary(BinaryExpr),
        Grouping(GroupingExpr),
        Literal(LiteralExpr),
        Unary(UnaryExpr),
    }

    pub trait Visitor<T> {
        fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
        fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
        fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
        fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    }

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
        pub left : ExprBoxed,
        pub operator : Token,
        pub right : ExprBoxed,
    }

    impl BinaryExpr {
        pub fn new(left : ExprBoxed, operator : Token, right : ExprBoxed) -> Self {
            Self {
                left,
                operator,
                right,
            }
        }
    }


    pub struct GroupingExpr {
        pub expression : ExprBoxed,
    }

    impl GroupingExpr {
        pub fn new(expression : ExprBoxed) -> Self {
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
        pub right : ExprBoxed,
    }

    impl UnaryExpr {
        pub fn new(operator : Token, right : ExprBoxed) -> Self {
            Self {
                operator,
                right,
            }
        }
    }

}
