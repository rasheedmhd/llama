use crate::token::Token;
use crate::token_type::TokenType;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::expr::LiteralValue;

// TO DO
// Fix Error Impl

// TO DO
// Add support to Llamaʼs scanner for C-style /* ... */ block comments.
// + handle newlines in them. + nesting.
// Is adding support for nesting more work than you expected? Why?
// TO DO
// Use BufReader to improve Performance
// Eng a Comp -> p70
#[allow(dead_code)]
pub struct Scanner {
    source: String,
    // The start and current fields are offsets that index into the string.
    // points to the first character in the lexeme being scanned,
    start: usize,
    // points at the character currently being considered.
    current: usize,
    // tracks what source line current is on so we
    // can produce tokens that know their location
    // TO DO
    // refactor to use std::num::NonZeroUsize
    // https://doc.rust-lang.org/std/num/type.NonZeroUsize.html
    line: usize,
    tokens: Vec<Token>,
}

// TO DO
// Use OnceLock
// It is now possible to easily replicate this crate's functionality in Rust's standard library with std::sync::OnceLock. The example above could be also be written as:

// use std::collections::HashMap;
// use std::sync::OnceLock;
// https://github.com/rust-lang-nursery/lazy-static.rs
// https://doc.rust-lang.org/std/sync/struct.OnceLock.html

// fn hashmap() -> &'static HashMap<u32, &'static str> {
//     static HASHMAP: OnceLock<HashMap<u32, &str>> = OnceLock::new();
//     HASHMAP.get_or_init(|| {
//         let mut m = HashMap::new();
//         m.insert(0, "foo");
//         m.insert(1, "bar");
//         m.insert(2, "baz");
//         m
//     })
// }
lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);
        keywords
    };
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: String::new(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
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
            Some(' ') | Some('\r') | Some('\t') => {}
            // TO DO
            // Behaving weird it repl mode
            Some('\n') => self.line += 1,
            Some('"') => self.string(),
            _ => {
                if self.is_digit(char.unwrap()) {
                    self.number();
                } else if self.is_alpha(char) {
                    self.identifier();
                }
                crate::repl::Llama::error(
                    Token::new(TokenType::EOF, "".to_string(), self.line, None),
                    "Unexpected Character",
                )
            }
        }
    }

    fn identifier(&mut self) {
        // while self.peek().is_alphanumeric() {

        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let identifier_literal = self.source[self.start..self.current].to_string();
        let mut token_type = KEYWORDS.get(&self.source[self.start..self.current]);
        if token_type.is_none() {
            token_type = Some(&TokenType::IDENTIFIER);
        }
        self.add_token_with_literal(token_type.unwrap().clone(), Some(LiteralValue::r#String(identifier_literal)))
    }

    fn is_alpha(&self, char: Option<char>) -> bool {
        return (char.unwrap() >= 'a' && char.unwrap() <= 'z')
            || (char.unwrap() >= 'A' && char.unwrap() <= 'Z')
            || char.unwrap() == '_';
    }

    fn is_alphanumeric(&self, char: char) -> bool {
        self.is_alpha(Some(char)) || self.is_digit(char)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // TO DO
    // Remove the Option and return a
    // and handle the None returned by
    // chars.nth()
    fn advance(&mut self) -> Option<char> {
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
        // nth() will return None if n is greater than or equal to the length of the iterator.
        // Note that all preceding elements, as well as the returned element, will be consumed from the iterator. That means that the preceding elements will be discarded, and also that calling nth(0) multiple times on the same iterator will return different elements.
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
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    // TO DO
    // Make llama recognize emojis 😂 in strings
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            crate::repl::Llama::error(
                Token::new(TokenType::EOF, "".to_string(), self.line, None),
                "Unterminated string.",
            );
        }
        // The closing "
        self.advance();
        // Trim the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::STRING, Some(LiteralValue::r#String(value)))
    }

    fn is_digit(&self, char: char) -> bool {
        char >= '0' && char <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the "."
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let number = self.source.to_string();
        // let number = self.source[self.start+1..self.current-1].parse::<f64>().unwrap();
        // self.add_token(TokenType::NUMBER);
        self.add_token_with_literal(TokenType::NUMBER, Some(LiteralValue::r#String(number)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    // The lexemes are only the raw substrings of the source code.
    // However, in the process of grouping character sequences into lexemes,
    // we also stumble upon some other useful information.
    // When we take the lexeme and bundle it together with that other data,
    // the result is a token. It includes useful stuff like:
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal));
    }
}
