
use crate::expr::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, ASTVisitor, Accept};
use crate::expr::ast::Expr;
use crate::expr::LiteralValue;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct AstPrinter;

impl AstPrinter {

    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&mut self, name: &str, exprs: &[Box<Expr>]) -> String {
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
                                literal: LiteralValue::Nil
                            } ,
                            right: Box::new(
                                Expr::Literal(
                                    LiteralExpr { value:  LiteralValue::r#String("123".to_string())  }
                                )
                            )
                        }
                    )
                ),
                operator: Token {
                    token_type: TokenType::STAR,
                    lexeme: "*".to_string(),
                    line: 1,
                    literal: LiteralValue::Nil
                } ,
                right: Box::new(
                    Expr::Grouping(
                        GroupingExpr {
                            expression: Box::new(
                                Expr::Literal(
                                    LiteralExpr { value:  LiteralValue::r#String("123".to_string())  }
                                )
                            )
                        }
                    )
                )
            }
        ));

        println!(" Expression: {}", expression);
        let expressions: &[Box<Expr>] = &[expression.clone()]; // Create a slice with one element
        let expression_as_a_string = AstPrinter::parenthesize(&mut AstPrinter, "", expressions);

        // println!(" Parenthesized Expression: {expression}");
        // string
        expression_as_a_string
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
        if Some(expr.value.clone()).is_some() {
            // return LiteralValue::Nil;
            return "Nil".to_string();
        }
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        AstPrinter.parenthesize(&expr.operator.lexeme, &[expr.right.clone()])
    }

}
