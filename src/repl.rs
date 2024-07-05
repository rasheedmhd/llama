#[allow(unused_variables)]

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;

use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::runtime_error::RuntimeError;
use crate::token_type::TokenType;

// TO DO
// Use AtomicBool
// https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html
static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub struct Llama {
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
            Llama::run_prompt();
        }
    }

    pub fn run(source: String) {
        // There is a Scanner::new associated method
        // I try to make to follow Rust's method/function name convention
        // so Scanner::new only declares the Scanner
        // I use Scanner::from to initialize the Scanner, with the source
        // of the input language
        let mut scanner   = Scanner::from(source);
        let tokens     = scanner.scan_tokens();
        // let mut parser     = Parser::new(tokens);
        // let expression  = parser.parse().unwrap();
        // let mut interpreter = Interpreter::new();
        for token in &tokens {
            println!("{:?}", token);
        }
        unsafe {
            if HAD_ERROR { return };
        }
                
        // interpreter.interpret(&expression);
        
    }


    // Llama is a scripting language, which means it executes directly from source.
    // Our interpreter supports two ways of running code. If you start llama from the
    // command line and give it a path to a file, it reads the file and executes it
    fn run_file(path: String) {
        // TO DO
        // Handle file error properly
        let code = fs::read_to_string(path).expect("File doesn't exist");
        Llama::run(code);
        unsafe {
            if HAD_ERROR { process::exit(65)}
            if HAD_RUNTIME_ERROR { process::exit(70)}
        }
    }

    fn run_prompt() {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut code_snippet = String::new();
            stdin().read_line(&mut code_snippet).unwrap();
            Llama::run(code_snippet.clone());
            // We need to reset this flag in the interactive loop.
            // If the user makes a mistake, it shouldn’t kill their entire session.
            unsafe {
                HAD_ERROR = false;
            }
        }
    }

    // Error Handling
    // Example
    // Error: Unexpected "," in argument list.
    // 15 | function(first, second,);
    //                            ^-- Here

    #[allow(dead_code)]
    fn report(line: usize, location: String,  message: &str) {
        eprintln!("line {line} Error {location}: {message}");
        unsafe {
            HAD_ERROR = true;
        }
    }

    pub fn error(token: Token,  message: &str) {
        if token.token_type == TokenType::EOF {
            Llama::report(token.line, "at end".to_string(),  message);
        } else {
            Llama::report(token.line, format!("at '{}'", token.lexeme ),  message);

        }
    }

    pub fn runtime_error(error: RuntimeError) {
        println!("Error: {:?}  \n[ Line {:?} ]", error.token.line, error.msg);
        unsafe {
            HAD_RUNTIME_ERROR = true;
        }
    }


}