use crate::token_type::TokenType;
use crate::token::Token;
// To Do
use crate::repl::Llama;


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
    pub tokens:     Vec<Token>,
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
            source,
            start:      0,
            current:    0,
            line:       1,
            tokens:     Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Starting at the first character of the source code,
    // the scanner figures out what lexeme the character belongs to, 
    // and consumes it and any following characters that are part of that lexeme. 
    // When it reaches the end of that lexeme, it emits a token.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(TokenType::EOF, "".to_string(), self.line, None);
        self.tokens.push(eof_token);
        self.tokens.clone()
    }

    // Recognizing Lexemes
    // The lexemes are only the raw substrings of the source code. 
    // [var] [language] [=] ["Llama"] [;]
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
            // To Do
            // _   => Llama.error(self.line, "Unexpected Character"),
            _   => eprintln!("{}, Unexpected Character", self.line),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next_char = self.source.chars().nth(self.current);
        self.current += 1;
        next_char
    }
    
    // The lexemes are only the raw substrings of the source code. 
    // However, in the process of grouping character sequences into lexemes, 
    // we also stumble upon some other useful information. 
    // When we take the lexeme and bundle it together with that other data, 
    // the result is a token. It includes useful stuff like:
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String> ) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, literal));
    }
}