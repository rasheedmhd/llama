#[allow(unused_variables)]
#[allow(dead_code)]

use std::env;
use std::fs;
use std::io::stdin;
use std::process;
use llama::token_type::TokenType;

static mut HAD_ERROR: bool = false;


fn main() {
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    Llama::start();
}

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

    
    // Llama is a scripting language, which means it executes directly from source. 
    // Our interpreter supports two ways of running code. If you start llama from the
    // command line and give it a path to a file, it reads the file and executes it
    
    // private static void runFile(String path) throws IOException {
    // byte[] bytes = Files.readAllBytes(Paths.get(path));
    // run(new String(bytes, Charset.defaultCharset()));
    // }
    fn run_file(path: String) {
        let code = fs::read_to_string(path).expect("file doesn't exist");
        Llama::run(code);
    }
    
    fn run_prompt() {
        loop {
            println!(" > ");
            let mut input_stream_reader = String::new();
            stdin().read_line(&mut input_stream_reader).expect("Enter Command");
            Llama::run(input_stream_reader.clone());
            // We need to reset this flag in the interactive loop. 
            // If the user makes a mistake, it shouldn’t kill their entire session.
            unsafe {
                HAD_ERROR = false;
            }
        }
    }

    fn run(source: String) {
        let scanner = Scanner::from(source);
        let tokens= scanner.scan_tokens();
        for token in &tokens {
            println!("{:#?}", token);
        }
    }

    // Error Handling
    // Example
    // Error: Unexpected "," in argument list.
    // 15 | function(first, second,);
    //                            ^-- Here
    fn error(line: u64, message: String) {
        Llama::report(line, "".to_string(),  message);
    }

    fn report(line: u64, location: String,  message: String) {

        eprintln!("[line {line}] Error {location}: {message}]");

        unsafe {
            HAD_ERROR = true;
        }
    }

}

struct Scanner {

}


impl Scanner {
    fn new() -> Self {
        Scanner {}
    }
    fn from(source: String) -> Self {
        Scanner {}
    }

    fn scan_tokens(&self) -> Vec<TokenType> {
        let tokens: Vec<TokenType> = Vec::new();
        tokens
    }
}



