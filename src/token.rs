use crate::expr::LiteralValue;
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
    // The current number scanning stores number literals.
    // A number literal is a series of digits optionally followed
    // by a . and one or more trailing digits as Some("number_literal")
    // A number literal in regex
    // [0-9]+[.]?[0-9]+
    // Subsequent code using the number literal should convert it into a float 
    // That is how Llama store numbers in memory  
    pub literal: LiteralValue,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.literal
        )
    }
}

impl Token {
    #[allow(dead_code)]
    pub fn new(token_type: TokenType, lexeme: String, line: usize, literal: Option<LiteralValue>) -> Self {
        let literal = literal.unwrap_or_else(|| LiteralValue::Nil);
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(&self, token_type: TokenType, lexeme: String, literal: LiteralValue) -> String {
        // TO DO
        let token_string: String = format!("{:?} {} {}", token_type, lexeme, literal);
        token_string
    }
}

