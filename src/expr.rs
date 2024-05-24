pub mod llama_ast {

    use llama::token::Token;

    pub enum Expr {
        Binary(BinaryExpr),
        Grouping(GroupingExpr),
        Literal(LiteralExpr),
        Unary(UnaryExpr),
    }

    // pub trait Visitor<T> {
    //     fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T {}
    //     fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T {}
    //     fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T {}
    //     fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T {}
    // }

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

    // impl Visitor<T> for BinaryExpr<T> {
    //     fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    //         visitor.visit_binary_expr(self)
    //     }
    // }

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

    // impl<T> Visitor<T> for GroupingExpr {
    //     fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    //         visitor.visit_grouping_expr(self)
    //     }
    // }

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

    // impl Visitor<T> for LiteralExpr {
    //     fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    //         visitor.visit_literal_expr(self)
    //     }
    // }

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

    // impl Visitor<T> for UnaryExpr {
    //     fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    //         visitor.visit_unary_expr(self)
    //     }
    // }

}
