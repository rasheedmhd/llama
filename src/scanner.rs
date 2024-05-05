use crate::token_type::TokenType;
use crate::token::Token;

// import static com.craftinginterpreters.lox.TokenType.*;
// class Scanner {
// private final String source;
// private final List<Token> tokens = new ArrayList<>();
// Scanner(String source) {
// this.source = source;
// }
// }
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
        self.tokens.push(eof_token);
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}


// List<Token> scanTokens() {
//     while (!isAtEnd()) {
//     // We are at the beginning of the next lex
//     start = current;
//     scanToken();
//     } 
//     tokens.add(new Token(EOF, "", null, line));
//     return tokens;
// }