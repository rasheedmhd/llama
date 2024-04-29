#[allow(unused_variables)]
use std::env::{self, args};
use std::io::stdin;
use std::process;

fn main() {
    // package com.craftinginterpreters.lox;
    // import java.io.BufferedReader;
    // import java.io.IOException;
    // import java.io.InputStreamReader;
    // import java.nio.charset.Charset;
    // import java.nio.file.Files;
    // import java.nio.file.Paths;
    // import java.util.List;

    // public class Lox {
    // public static void main(String[] args) throws IOException {
    // if (args.length > 1) {
    //     System.out.println("Usage: jlox [script]");
    //         System.exit(64);
    //         } else if (args.length == 1) {
    //             runFile(args[0]);
    //         } else {
    //             runPrompt();
    //         }
    //     }
    // }

    // Baby Steps
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    get();
}

// Exit Codes
// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
pub fn get() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: llama {:#}", args[1]);
        process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(v: &String) {
    // unimplemented!()
}
fn run_prompt() {
    // unimplemented!()
}
