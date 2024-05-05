use crate::token_type::TokenType;
// use std::any::Any;
#[allow(dead_code)]

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    // TO DO
    // literal: dyn Any,  
    // create an enum to handle all TokenLiteral types
    literal: Option<String>,  
}

impl Token {
    #[allow(dead_code)]
    pub fn new(token_type: TokenType, lexeme: String, line: usize, literal: Option<String>) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(&self, token_type: TokenType, lexeme: String, literal: String) -> String {
        // TO DO
        let token_string: String = format!("{:?} {} {}", token_type, lexeme, literal);
        token_string
    }
}

// fn print_type(value: &dyn Any) {
//     if let Some(v) = value.downcast_ref::<i32>() {
//         println!("This is an i32: {}", v);
//     } else if let Some(v) = value.downcast_ref::<String>() {
//         println!("This is a String: {}", v);
//     } else {
//         println!("Unknown type");
//     }
// }


