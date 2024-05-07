#[allow(unused_variables)]

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;

use crate::scanner::Scanner;

static mut HAD_ERROR: bool = false;


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

    fn run(source: String) {
        let mut scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    
    // Llama is a scripting language, which means it executes directly from source. 
    // Our interpreter supports two ways of running code. If you start llama from the
    // command line and give it a path to a file, it reads the file and executes it
    fn run_file(path: String) {
        let code = fs::read_to_string(path).expect("File doesn't exist");
        Llama::run(code);
    }
    
    fn run_prompt() {
        loop {
            print!("> ");
            let _ = stdout().flush();
            // stdin().read_line(&mut code_snippet.trim()).unwrap();
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
    pub fn error(line: usize, message: String) {
        Llama::report(line, "".to_string(),  message);
    }

    #[allow(dead_code)]
    fn report(line: usize, location: String,  message: String) {
        eprintln!("line {line} Error {location}: {message}");
        unsafe {
            HAD_ERROR = true;
        }
    }

}