
use crate::expr::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, ASTVisitor, Accept};
use crate::expr::ast::Expr;
use crate::token::Token;
use crate::token_type::TokenType;
struct AstPrinter;

impl AstPrinter {

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
    
}

impl ASTVisitor<String> for  AstPrinter {
    // fn print(expr: Expr) {
    //     expr.accept(self)
    // }
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        AstPrinter.parenthesize(&expr.operator.lexeme, &[expr.left.clone(), expr.left.clone()])
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        AstPrinter.parenthesize("group", &[expr.expression.clone()])
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        // if (expr.value == null) return "nil";
        // return expr.value.toString();
        if expr.value.is_empty() {
            return "nil".to_string();
        }
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        AstPrinter.parenthesize(&expr.operator.lexeme, &[expr.right.clone()])
    }

    // private String parenthesize(String name, Expr... exprs) {
    //     StringBuilder builder = new StringBuilder();
    //     builder.append("(").append(name);
    //     for (Expr expr : exprs) {
    //     builder.append(" ");
    //     builder.append(expr.accept(this));
    //     }
    //     builder.append(")");
    //     return builder.toString();
    // }


}

// public static void main(String[] args) {
//     Expr expression = new Expr.Binary(
//     new Expr.Unary(
//     new Token(TokenType.MINUS, "-", null, 1),
//     new Expr.Literal(123)),
//     new Token(TokenType.STAR, "*", null, 1),
//     new Expr.Grouping(
//     new Expr.Literal(45.67)));

fn main() -> Expr {
    let expression = Expr::Binary(
        BinaryExpr { 
            left: Box::new(
                Expr::Unary(
                    UnaryExpr { 
                        operator: Token { 
                            token_type: TokenType::MINUS, lexeme: "-".to_string(), 
                            line: 1, 
                            literal: Some("-".to_string())  
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
                token_type: TokenType::STAR, lexeme: "*".to_string(), 
                line: 1, 
                literal: Some("*".to_string())  
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
    );

    expression
}