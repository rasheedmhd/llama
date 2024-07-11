pub mod stmt {
    use crate::expr::ast::Expr;
    pub enum Stmt {
        Expression(ExpressionStmt),
        Print(PrintStmt),
    }

     pub trait Visitor<T> {
        fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
        fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    }

    impl Stmt {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
                Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            }
        }
    }


    pub struct ExpressionStmt {
        pub expression : Expr,
    }

    impl ExpressionStmt {
        pub fn new(expression : Expr) -> Self {
            Self {
                expression,
            }
        }
    }


    pub struct PrintStmt {
        pub expression : Expr,
    }

    impl PrintStmt {
        pub fn new(expression : Expr) -> Self {
            Self {
                expression,
            }
        }
    }
}