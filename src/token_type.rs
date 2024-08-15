use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftPAREN,  // (
    RightPAREN, // )
    LeftBRACE,  // {
    RightBRACE, // }
    COMMA,      // ,
    DOT,        // .
    MINUS,      // -
    PLUS,       // +
    SEMICOLON,  // ;
    SLASH,      // /
    STAR,       // *

    // One or two character tokens.
    BANG,         // !
    BangEQUAL,    // !=
    EQUAL,        // =
    EqualEQUAL,   // ==
    GREATER,      // >
    GreaterEQUAL, // >=
    LESS,         // <
    LessEQUAL,    // <=

    // Literals.
    // IDENTIFIER "" 1
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,    // and
    CLASS,  // class
    ELSE,   // else
    FALSE,  // false
    FUN,    // fun
    FOR,    // for
    IF,     // if
    NIL,    // nil
    OR,     // or
    PRINT,  // print
    RETURN, // return
    SUPER,  // super
    THIS,   // this
    TRUE,   // true
    VAR,    // var
    WHILE,  // while

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_str = match self {
            TokenType::LeftPAREN => "(",
            TokenType::RightPAREN => ")",
            TokenType::LeftBRACE => "{",
            TokenType::RightBRACE => "}",
            TokenType::COMMA => ",",
            TokenType::DOT => ".",
            TokenType::MINUS => "-",
            TokenType::PLUS => "+",
            TokenType::SEMICOLON => ";",
            TokenType::SLASH => "/",
            TokenType::STAR => "*",
            TokenType::BANG => "!",
            TokenType::BangEQUAL => "!=",
            TokenType::EQUAL => "=",
            TokenType::EqualEQUAL => "==",
            TokenType::GREATER => ">",
            TokenType::GreaterEQUAL => ">=",
            TokenType::LESS => "<",
            TokenType::LessEQUAL => "<=",
            TokenType::IDENTIFIER => "identifier",
            TokenType::STRING => "string",
            TokenType::NUMBER => "number",
            TokenType::AND => "and",
            TokenType::CLASS => "class",
            TokenType::ELSE => "else",
            TokenType::FALSE => "false",
            TokenType::FUN => "fun",
            TokenType::FOR => "for",
            TokenType::IF => "if",
            TokenType::NIL => "nil",
            TokenType::OR => "or",
            TokenType::PRINT => "print",
            TokenType::RETURN => "return",
            TokenType::SUPER => "super",
            TokenType::THIS => "this",
            TokenType::TRUE => "true",
            TokenType::VAR => "var",
            TokenType::WHILE => "while",
            TokenType::EOF => "EOF",
        };
        write!(f, "{}", token_str)
    }
}
