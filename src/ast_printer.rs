
use crate::expr::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, ASTVisitor, Accept};
use crate::expr::ast::Expr;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct AstPrinter;

impl AstPrinter {

    // pub fn print(&mut self, expr: Expr) -> String {
    //     expr.accept(self)
    // }

    fn parenthesize(&mut self, name: &str, exprs: &[Box<Expr>]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }

    pub fn main() -> String {
        let expression = Box::new(Expr::Binary(
            BinaryExpr { 
                left: Box::new(
                    Expr::Unary(
                        UnaryExpr { 
                            operator: Token { 
                                token_type: TokenType::MINUS,
                                lexeme: "-".to_string(),
                                line: 1,
                                literal: None
                            } ,
                            right: Box::new(
                                Expr::Literal(
                                    LiteralExpr { value: "123".to_string() }
                                )
                            )
                        }
                    )
                ),
                operator: Token {
                    token_type: TokenType::STAR,
                    lexeme: "*".to_string(),
                    line: 1,
                    literal: None
                } ,
                right: Box::new(
                    Expr::Grouping(
                        GroupingExpr {
                            expression: Box::new(
                                Expr::Literal(
                                    LiteralExpr { value: "45.67".to_string() }
                                )
                            )
                        }
                    )
                )
            }
        ));

        println!(" Expression: {}", expression);
        let expressions: &[Box<Expr>] = &[expression.clone()]; // Create a slice with one element
        let string = AstPrinter::parenthesize(&mut AstPrinter, "", expressions);

        println!(" Parenthesized Expression: {string}");
        string
        // expression
    }
    
}

impl ASTVisitor<String> for  AstPrinter {

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        AstPrinter.parenthesize(&expr.operator.lexeme, &[expr.left.clone(), expr.right.clone()])
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        AstPrinter.parenthesize("group", &[expr.expression.clone()])
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        if expr.value.is_empty() {
            return "nil".to_string();
        }
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        AstPrinter.parenthesize(&expr.operator.lexeme, &[expr.right.clone()])
    }

}
