use crate::interpreter::Interpreter;
use crate::runtime_error::RuntimeError;
use crate::token::Token;

type BoxedExpr = Box<Expr>;
type LiteralResult = Result<Literal, RuntimeError>;

// EXPRESSIONS
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
    Logical(LogicalExpr),
    Call(CallExpr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: BoxedExpr,
    pub operator: Token,
    pub right: BoxedExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupingExpr {
    pub expression: BoxedExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Function(Callable),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Callable;

impl Callable {
    pub fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Literal>) -> LiteralResult {
        todo!()
    }
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
        if let Self::Bool(left) = self {
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
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Self::String(str) => &str,
            _ => panic!(),
        }
    }

    pub fn unwrap_num(&self) -> f64 {
        match self {
            Self::Number(num) => *num,
            _ => panic!(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Nil => false,
            Literal::Bool(bool_value) => *bool_value,
            _ => true,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = match self {
            Literal::String(string) => {
                write!(f, "{}", string)
            }
            Literal::Number(num) => {
                return if num.fract() == 0.0 {
                    write!(f, "{:?}", *num as i64)
                } else {
                    write!(f, "{:?}", num)
                };
            }
            Literal::Bool(bool) => {
                write!(f, "{}", bool)
            }
            Literal::Nil => {
                write!(f, "nil")
            }
            Literal::Function(_) => {
                write!(f, "<Function>")
            }
        };
        literal
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralExpr {
    pub value: Literal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: BoxedExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableExpr {
    pub name: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignExpr {
    pub name: Token,
    pub value: BoxedExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogicalExpr {
    pub left: BoxedExpr,
    pub operator: Token,
    pub right: BoxedExpr,
}

// EXPRESSION AST Nodes' new() IMPLEMENTATIONS
// Takes the Expression's constituent parts as arguments
// and creates a new Expression initializing it
// with the passed arguments.
impl BinaryExpr {
    pub fn new(left: BoxedExpr, operator: Token, right: BoxedExpr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl GroupingExpr {
    pub fn new(expression: BoxedExpr) -> Self {
        Self { expression }
    }
}

impl LiteralExpr {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: BoxedExpr) -> Self {
        Self { operator, right }
    }
}

impl VariableExpr {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl AssignExpr {
    pub fn new(name: Token, value: BoxedExpr) -> Self {
        Self { name, value }
    }
}

impl LogicalExpr {
    pub fn new(left: BoxedExpr, operator: Token, right: BoxedExpr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpr {
    pub callee: BoxedExpr,
    pub paren: Token,
    pub arguments: Vec<BoxedExpr>,
}

impl CallExpr {
    pub fn new(callee: BoxedExpr, paren: Token, arguments: Vec<BoxedExpr>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

pub trait Visitor<T> {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> T;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> T;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> T;
    fn visit_call_expr(&mut self, expr: &CallExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
            Expr::Call(expr) => visitor.visit_call_expr(expr),
        }
    }
}