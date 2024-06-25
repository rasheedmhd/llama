// To Do
// Type refactor for literal value
#[derive(Clone, Debug)]
pub enum LiteralValue {
    r#String(String),
    Number(f64),
    Bool(bool),
    Nil
}


pub mod ast {
    use std::fmt;
    use crate::expr::LiteralValue;

    type ExprBoxed = Box<Expr>;

    use crate::token::Token;

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Debug)]
    pub struct LiteralExpr {
        pub value : LiteralValue,
    }

    impl LiteralExpr {
        pub fn new(value : LiteralValue) -> Self {
            Self {
                value,
            }
        }
    }

    #[derive(Clone, Debug)]
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

    // FORMATTING
    impl fmt::Display for BinaryExpr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write!(f, "({} {} {})", self.left, self.operator, self.right)
            // Reverse Police Notation
            write!(f, "({} {} {})", self.left, self.right, self.operator)
        }
    }

    impl fmt::Display for GroupingExpr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(group {})", self.expression)
        }
    }

    impl fmt::Display for LiteralExpr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl fmt::Display for LiteralValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    impl fmt::Display for UnaryExpr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})", self.operator, self.right)
        }
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Binary(expr) => write!(f, "({} {} {})", expr.operator.lexeme, expr.left,  expr.right),
                Expr::Unary(expr) => write!(f, "({} {})", expr.operator.lexeme, expr.right),
                Expr::Literal(expr) => write!(f, "{}", expr.value),
                Expr::Grouping(expr) => write!(f, "(group {})", expr.expression),
            }
        }
    }


}



