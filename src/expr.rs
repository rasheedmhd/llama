pub mod ast {
    use crate::token::Token;

    type BoxedExpr = Box<Expr>;
    // EXPRESSIONS
    #[derive(Clone, Debug, PartialEq)]
    pub enum Expr {
        Binary(BinaryExpr),
        Grouping(GroupingExpr),
        Literal(LiteralExpr),
        Unary(UnaryExpr),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct BinaryExpr {
        pub left : BoxedExpr,
        pub operator : Token,
        pub right : BoxedExpr,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupingExpr {
        pub expression : BoxedExpr,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Literal {
        String(String),
        Number(f64),
        Bool(bool),
        Nil
    }

    impl Literal {
        pub fn is_num(&self) -> bool {
            match self {
                Self::Number(_) => true,
                _ => false
            }
        }
        pub fn is_string(&self) -> bool {
           match self {
                Self::String(_) => true,
                _ => false
            }
        }
        pub fn unwrap_num(&self) -> f64 {
            match self {
                Self::Number(num) => *num,
                // hopefully unreachable
                // To Do
                // proper handling later
                _ => panic!()
            }
        }
        pub fn wrap_num(value: f64) -> Self {
            Self::Number(value)
        }
    }

    impl std::fmt::Display for Literal {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let literal = match self {
                Literal::String(string) => { write!(f, "{}", string) },
                Literal::Number(num) => { write!(f, "{:?}", num) },
                Literal::Bool(bool) => { write!(f, "{}", bool) },
                Literal::Nil => { write!(f, "nil") },
            };
            literal
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct LiteralExpr {
        pub value : Literal,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct UnaryExpr {
        pub operator : Token,
        pub right : BoxedExpr,
    }

    // EXPRESSION new IMPLEMENTATIONS
    // Takes the Expression's constituent parts as arguments
    // and creates a new Expression initializing it
    // with the passed arguments.
    impl BinaryExpr {
        pub fn new(left : BoxedExpr, operator : Token, right : BoxedExpr) -> Self {
            Self {
                left,
                operator,
                right,
            }
        }
    }

    impl GroupingExpr {
        pub fn new(expression : BoxedExpr) -> Self {
            Self {
                expression,
            }
        }
    }

    impl LiteralExpr {
        pub fn new(value : Literal) -> Self {
            Self {
                value,
            }
        }
    }

    impl UnaryExpr {
        pub fn new(operator : Token, right : BoxedExpr) -> Self {
            Self {
                operator,
                right,
            }
        }
    }
}