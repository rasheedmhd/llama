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
        pub fn is_equal(&self, other: &Self) -> bool {
            if let Self::Nil = self {
                if let Self::Nil = other {
                    return true;
                }
            }
            if let Self::Number(left) = self {
                if let Self::Number(right) = other {
                    return left == right;
                }
            }
            if let Self::Bool(left ) = self {
                if let Self::Bool(right) = other {
                    return left == right;
                }
            }
            if let Self::String(left) = self {
                if let Self::String(right) = other {
                    return left == right;
                }
            }
            return false;
        }

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

        pub fn unwrap_string(&self) -> &str {
            match self {
                Self::String(str) => &str,
                _ => panic!()
            }
        }

        pub fn unwrap_num(&self) -> f64 {
            match self {
                Self::Number(num) => *num,
                _ => panic!()
            }
        }

        pub fn is_truthy(&self) -> bool {
            match self {
                Literal::Nil => false,
                Literal::Bool(bool_value) => *bool_value,
                _ => true
            }
        }
    }

    impl std::fmt::Display for Literal {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let literal = match self {
                Literal::String(string) => { write!(f, "{}", string) },
                Literal::Number(num) => {
                    return if num.fract() == 0.0 {
                        write!(f, "{:?}", *num as i64)
                    } else {
                        write!(f, "{:?}", num)
                    };
                },
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
    pub struct  UnaryExpr {
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