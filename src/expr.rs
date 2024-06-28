pub mod ast {

    use crate::token::Token;
    type ExprBoxed = Box<Expr>;

    // EXPRESSIONS
    #[derive(Clone, Debug)]
    pub enum Expr {
        Binary(BinaryExpr),
        Grouping(GroupingExpr),
        Literal(LiteralExpr),
        Unary(UnaryExpr),
    }

    #[derive(Clone, Debug)]
    pub struct BinaryExpr {
        pub left : ExprBoxed,
        pub operator : Token,
        pub right : ExprBoxed,
    }

    #[derive(Clone, Debug)]
    pub enum LiteralValue {
        r#String(String),
        Number(f64),
        Bool(bool),
        Nil
    }

    #[derive(Clone, Debug)]
    pub struct GroupingExpr {
        pub expression : ExprBoxed,
    }

    #[derive(Clone, Debug)]
    pub struct LiteralExpr {
        pub value : LiteralValue,
    }

    #[derive(Clone, Debug)]
    pub struct UnaryExpr {
        pub operator : Token,
        pub right : ExprBoxed,
    }

    // EXPRESSION new IMPLEMENTATIONS
    // Takes in the Expression's constituent parts
    // and create a new Expression initializing it
    // with the passed arguments.
    impl BinaryExpr {
        pub fn new(left : ExprBoxed, operator : Token, right : ExprBoxed) -> Self {
            Self {
                left,
                operator,
                right,
            }
        }
    }

    impl GroupingExpr {
        pub fn new(expression : ExprBoxed) -> Self {
            Self {
                expression,
            }
        }
    }

    impl LiteralExpr {
        pub fn new(value : LiteralValue) -> Self {
            Self {
                value,
            }
        }
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



