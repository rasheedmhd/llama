use crate::expr::Literal;
use crate::token_type::TokenType;
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Literal,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    #[allow(dead_code)]
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<Literal>,
    ) -> Self {
        let literal = literal.unwrap_or_else(|| Literal::Nil);
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(&self, token_type: TokenType, lexeme: String, literal: Literal) -> String {
        let token_string: String = format!("{:?} {} {:?}", token_type, lexeme, literal);
        token_string
    }
}
