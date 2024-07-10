use crate::token::Token;
use crate::token_type::TokenType;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::expr::ast::Literal;

// TO DO
// Add support to Llamaʼs scanner for C-style /* ... */ block comments.
// + handle newlines in them. + nesting.
// Is adding support for nesting more work than you expected? Why?
// TO DO
// Use BufReader to improve Performance
// Eng a Comp -> p70
#[allow(dead_code)]
pub struct Scanner {
    // Text comes into the scanner from source and
    // get transformed into tokens in  stored
    // in tokens
    source: String,
    // The start and current fields are offsets that index into the string.
    // points to the first character in the lexeme being scanned,
    start: usize,
    // points at the character currently being considered.
    current: usize,
    // Tracks which line in source we are on
    // So we can produce tokens that know their location
    line: usize,
    // Serves as a source for the next step of the compilation
    // pipeline: Tokens move from here into the Parser
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
            '(' => self.add_token(TokenType::LeftPAREN),
            ')' => self.add_token(TokenType::RightPAREN),
            '{' => self.add_token(TokenType::LeftBRACE),
            '}' => self.add_token(TokenType::RightBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            // Operators
            // To Do
            // Add a ++ for adding 1 to a value
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEQUAL)
                } else if !self.match_char('=') {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEQUAL)
                } else if !self.match_char('=') {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEQUAL)
                } else if !self.match_char('=') {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEQUAL)
                } else if !self.match_char('=') {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if !self.match_char('/') {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(char) {
                    self.number();
                } else if self.is_alpha(char) {
                    self.identifier();
                } else {
                    crate::repl::Llama::error(
                        Token::new(TokenType::EOF, "".to_string(), self.line, None),
                        "Unexpected Character",
                    )
                }
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
        self.add_token_with_literal(token_type.unwrap().clone(), Some(Literal::String(identifier_literal)))
    }

    fn is_alpha(&self, char: char) -> bool {
        return (char >= 'a' && char <= 'z')
            || (char >= 'A' && char <= 'Z')
            || char == '_';
    }

    fn is_alphanumeric(&self, char: char) -> bool {
        self.is_alpha(char) || self.is_digit(char)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // TO DO
    // Remove the Option and return a
    // and handle the None returned by
    // chars.nth()
    fn advance(&mut self) -> char {
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
        // nth() will return None if n is greater than or equal to the length of the iterator.
        // Note that all preceding elements, as well as the returned element, will be consumed from the iterator.
        // That means that the preceding elements will be discarded,
        // and also that calling nth(0) multiple times on the same iterator will return different elements.
        let next_char = self.source.chars().nth(self.current).unwrap();
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
            // TO DO
            // Impl better error msgs
            // Eg:
            //
            // File "<stdin>", line 1
            // "hell
            //     ^
            // SyntaxError: unterminated string literal (detected at line 1)

            crate::repl::Llama::error(
                Token::new(TokenType::EOF, "".to_string(), self.line, None),
                "Unterminated string.",
            );
        }
        // The closing "
        self.advance();
        // Trim the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::STRING, Some(Literal::String(value)))
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
        let num = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token_with_literal(TokenType::NUMBER, Some(Literal::Number(num)))
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

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal));
    }
}