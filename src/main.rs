#[allow(unused_variables)]
use std::env;
use std::fs;
use std::io::{
    stdin, BufReader, Result,
};

use std::process;

fn main() {
    // Baby Steps
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    get();
}

// Exit Codes
// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
pub fn get() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: llama {:#?}", args);
        process::exit(64);
    } else if args.len() == 1 {
        run_file(args[0].clone());
    } else {
        run_prompt();
    }
}

// Llama is a scripting language, which means it executes directly from source. Our
// interpreter supports two ways of running code. If you start jlox from the
// command line and give it a path to a file, it reads the file and executes it

// ﻿private static void runFile(String path) throws IOException {
// byte[] bytes = Files.readAllBytes(Paths.get(path));
// run(new String(bytes, Charset.defaultCharset()));
// }
fn run_file(path: String) {
    let bytes = fs::read_to_string(path).expect("{path} doesn't exist");
    run(String::from(bytes));
}
fn run_prompt() -> Result<()> {
    let mut input_stream_reader = String::new();
    stdin().read_line(&mut input_stream_reader).expect("Enter Command");
    // for (;;) {
    //     System.out.print("> ");
    //     String line = reader.readLine();
    //     if (line == null) break;
    //     run(line);
    //     }
    loop {
        print!("> ");
        if input_stream_reader.is_empty() {
            break;
        }
        // run(input_stream_reader);
    }
    Ok(())
}
fn run(bytes: String) {
    // unimplemented!()
}
