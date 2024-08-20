use crate::expr::Expr;
use crate::token::Token;

type BoxedExpr = Box<Expr>;
type BoxedStmt = Box<Stmt>;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStmt {
    pub expression: BoxedExpr,
}
impl ExpressionStmt {
    pub fn new(expression: BoxedExpr) -> Self {
        Self { expression }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrintStmt {
    pub expression: BoxedExpr,
}
impl PrintStmt {
    pub fn new(expression: BoxedExpr) -> Self {
        Self { expression }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: BoxedExpr,
}
impl VarStmt {
    pub fn new(name: Token, initializer: BoxedExpr) -> Self {
        Self { name, initializer }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStmt {
    pub condition: BoxedExpr,
    pub then_branch: BoxedStmt,
    pub else_branch: Option<BoxedStmt>,
}

impl IfStmt {
    pub fn new(
        condition: BoxedExpr,
        then_branch: BoxedStmt,
        else_branch: Option<BoxedStmt>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStmt {
    pub condition: BoxedExpr,
    pub body: BoxedStmt,
}

impl WhileStmt {
    pub fn new(condition: BoxedExpr, body: BoxedStmt) -> Self {
        Self { condition, body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionStmt {
    pub name : Token,
    pub params: Vec<Token>,
    pub body : Vec<Stmt>,
}

impl FunctionStmt {
    pub fn new(name : Token, params: Vec<Token>, body : Vec<Stmt>) -> Self {
        Self {
            name,
            params,
            body,
        }
    }
}
pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> T;
    fn visit_function_stmt(&mut self, expr: &FunctionStmt) -> T;

}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
            Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),

        }
    }
}