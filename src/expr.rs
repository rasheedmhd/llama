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

    pub trait ASTVisitor<T> {
        fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
        fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
        fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
        fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    }
    pub trait Accept<T> {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T;
    }

    impl<T> Accept<T> for Expr {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T {
            match self {
                Expr::Binary(expr) => visitor.visit_binary_expr(expr),
                Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
                Expr::Literal(expr) => visitor.visit_literal_expr(expr),
                Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            }
        }
    }

    impl<T> Accept<T> for BinaryExpr {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T {
            visitor.visit_binary_expr(self)
        }
    }
    impl<T> Accept<T> for GroupingExpr {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T {
            visitor.visit_grouping_expr(self)
        }
    }
    impl<T> Accept<T> for LiteralExpr {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T {
            visitor.visit_literal_expr(self)
        }
    }
    impl<T> Accept<T> for UnaryExpr {
        fn accept<V: ASTVisitor<T>>(&self, visitor: &mut V) -> T {
            visitor.visit_unary_expr(self)
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
