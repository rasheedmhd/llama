use crate::token_type::TokenType;
#[allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    // TO DO
    // literal: dyn Any,  
    // create an enum to handle all TokenLiteral types
    // TO DO
    // The current number scanning stores number literals 
    // which are supposed to be A number literal is a series of digits optionally followed
    // by a . and one or more trailing digits as Some("number_literal")
    // Subsequent code using the number literal should convert it into a float 
    // That is how Llama store numbers in memory  
    pub literal: Option<String>,  
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.clone().unwrap_or_default()
        )
    }
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


