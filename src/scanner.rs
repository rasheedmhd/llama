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
            source,
            start:      0,
            current:    0,
            line:       1,
            tokens:     Vec::new(),
        }
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
    // To Do
    // Next Char returns wih a \n char which the scanner doesn't recognize
    // There throwing an error
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
            // Operators 
            // To Do
            // Add a ++ for adding 1 to a value
            Some('!') => {
                    if self.match_char('=') {
                        self.add_token(TokenType::BangEQUAL)
                    } else if !self.match_char('=') {
                        self.add_token(TokenType::BANG)
                    }
                }
            Some('=') => {
                    if self.match_char('=') {
                        self.add_token(TokenType::EqualEQUAL)
                    } else if !self.match_char('=') {
                        self.add_token(TokenType::EQUAL)
                    }
                }
            Some('<') => {
                    if self.match_char('=') {
                        self.add_token(TokenType::LessEQUAL)
                    } else if !self.match_char('=') {
                        self.add_token(TokenType::LESS)
                    }
                }
            Some('>') => {
                    if self.match_char('=') {
                        self.add_token(TokenType::GreaterEQUAL)
                    } else if !self.match_char('=') {
                        self.add_token(TokenType::GREATER)
                    }
                }
            Some('/') => {
                    if self.match_char('/') {
                        // A comment goes until the end of the line
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();                            
                        } 
                    } else if !self.match_char('/') {
                        self.add_token(TokenType::SLASH);
                    }
                }
            Some(' ') | Some('\r') | Some('\t')  => {},
            Some('\n') => self.line += 1,
            Some('"') => self.string(),
            // Some('0'..='9') => {
            //     self.number();
            // },
            _   => {
                if self.is_digit(char) {
                    self.number();
                }
                crate::repl::Llama::error(self.line, "Unexpected Character".to_string())
            },
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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
           return false;
        } 
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            crate::repl::Llama::error(self.line, "Unterminated string.".to_string());
        }
        // The closing "
        self.advance();
        // Trim the surrounding quotes.
        let value: String = self.source[self.start+1..self.current-1].to_string();
        self.add_token_with_literal(TokenType::STRING, Some(value))
    }

    fn is_digit(&self, char: Option<char>) ->  bool {
        char.unwrap() >= '0' && char.unwrap() <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(Some(self.peek())) {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the "."
            self.advance();
            while self.is_digit(Some(self.peek())) {
                self.advance();
            }
        }
        let number = self.source.to_string();
        // let number = self.source[self.start+1..self.current-1].parse::<f64>().unwrap();
        // self.add_token(TokenType::NUMBER);
        self.add_token_with_literal(TokenType::NUMBER, Some(number))
    }

    fn peek_next(&self) -> Option<char> {
        if self.current+1 >= self.source.len() {
            return Some('\0');
        }
        return self.source.chars().nth(self.current+1)
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
