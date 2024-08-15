use crate::environment::Environment;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;
use std::rc::Rc;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::runtime_error::RuntimeError;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;

// TO DO
// Use AtomicBool
// https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html
static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub struct Llama {}

pub struct Repl {
    interpreter: Interpreter,
    scanner: Scanner,
    parser: Parser,
}

impl Repl {
    fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            scanner: Scanner::new(),
            parser: Parser::new(),
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut code_snippet = String::new();
            stdin().read_line(&mut code_snippet).unwrap();
            self.run(code_snippet.clone());
        }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::from(tokens);
        let statements = match parser.parse() {
            Ok(statements) => statements,
            Err(e) => {
                eprintln!("Failed to parse expression: {}", e);
                return;
            }
        };

        unsafe {
            if HAD_ERROR {
                return;
            };
        }

        self.interpreter.interpret(statements);
    }
}

impl Llama {
    // Exit Codes
    // https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
    pub fn start() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
            println!("Usage: llama [script] {:?}", args.len());
            process::exit(64);
        } else if args.len() == 2 {
            Llama::run_file(args[1].clone());
        } else {
            Repl::new().run_prompt()
        }
    }

    // Llama is a scripting language, which means it executes directly from source.
    // Our interpreter supports two ways of running code. If you start llama from the
    // command line and give it a path to a file, it reads the file and executes it
    fn run_file(path: String) {
        // TO DO
        // Handle file error properly
        let code = fs::read_to_string(path).expect("File doesn't exist");
        Repl::new().run(code);
        unsafe {
            if HAD_ERROR {
                process::exit(65)
            }
            if HAD_RUNTIME_ERROR {
                process::exit(70)
            }
        }
    }

    #[allow(dead_code)]
    fn report(line: usize, location: String, message: &str) {
        eprintln!("line {line} Error {location}: {message}");
        unsafe {
            HAD_ERROR = true;
        }
    }

    pub fn error(token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            Llama::report(token.line, "at end".to_string(), message);
        } else {
            Llama::report(token.line, format!("at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(error: RuntimeError) {
        println!("{}", error);
        unsafe {
            HAD_RUNTIME_ERROR = true;
        }
    }
}
