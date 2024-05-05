use crate::token_type::TokenType;
use crate::token::Token;

#[allow(dead_code)]
pub struct Scanner {
    source:     String,
    // The start and current fields are offsets that index into the string. 
    // points to the first character in the lexeme being scanned,
    start:      usize,
    // points at the character currently being considered. 
    current:    usize,
    // tracks what source line current is on so we 
    // can produce tokens that know their location
    line:       usize,
    tokens:     Vec<Token>,
}


impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: String::new(),
            start:      0,
            current:    0,
            line:       1,
            tokens: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(source: String) -> Self {
        Self {
            source: source,
            start:      0,
            current:    0,
            line:       1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(TokenType::EOF, "".to_string(), self.line, None);
        let  mut tokens: Vec<Token> = Vec::new();
        tokens.push(eof_token);
        tokens
    }

    // Recognizing Lexemes
    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            Some('(') => self.add_token(TokenType::LeftPAREN),
            Some(')') => self.add_token(TokenType::RightPAREN),
            Some('{') => self.add_token(TokenType::LeftBRACE),
            Some('}') => self.add_token(TokenType::RightBRACE),
            Some(',') => self.add_token(TokenType::COMMA),
            Some('.') => self.add_token(TokenType::DOT),
            Some('-') => self.add_token(TokenType::MINUS),
            Some('+') => self.add_token(TokenType::PLUS),
            Some(';') => self.add_token(TokenType::SEMICOLON),
            Some('*') => self.add_token(TokenType::STAR),
            _   => todo!()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let next_char = self.source.chars().nth(self.current);
        self.current += 1;
        next_char
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String> ) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, literal));
    }
}
