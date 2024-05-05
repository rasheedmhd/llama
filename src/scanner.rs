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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        unimplemented!();
    }
}
