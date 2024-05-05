#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    // ( ) { } , . - + ; / *
    LeftPAREN, 
    RightPAREN, 
    LeftBRACE, 
    RightBRACE,
    COMMA, 
    DOT, 
    MINUS, 
    PLUS, 
    SEMICOLON,
    SLASH, 
    STAR,

    // One or two character tokens.
    // ! != = == > >= < <=
    BANG, 
    BangEQUAL,
    EQUAL, 
    EqualEQUAL,
    GREATER,
    GreaterEQUAL,
    LESS, 
    LessEQUAL,

    // Literals.
    // IDENTIFIER "" 1
    IDENTIFIER, 
    STRING, 
    NUMBER,

    // Keywords.
    AND, 
    CLASS, 
    ELSE, 
    FALSE, 
    FUN, 
    FOR, 
    IF, 
    NIL, 
    OR,
    PRINT, 
    RETURN, 
    SUPER, 
    THIS, 
    TRUE, 
    VAR, 
    WHILE,
    
    EOF
}