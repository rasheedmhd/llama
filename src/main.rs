#[allow(unused_variables)]
use std::env;
use std::fs;
use std::io::{
    stdin, Result
};
use std::process;

fn main() {
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    get();
}

// Exit Codes
// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
pub fn get() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        println!("Usage: llama [script]");
        process::exit(64);
    } else if args.len() == 1 {
        run_file(args[0].clone());
    } else {
        // run_prompt();
        process::exit(63)
    }
}

// Llama is a scripting language, which means it executes directly from source. 
// Our interpreter supports two ways of running code. If you start llama from the
// command line and give it a path to a file, it reads the file and executes it

// private static void runFile(String path) throws IOException {
// byte[] bytes = Files.readAllBytes(Paths.get(path));
// run(new String(bytes, Charset.defaultCharset()));
// }
fn run_file(path: String) -> Result<()>{
    let code = fs::read_to_string(path).expect("file doesn't exist");
    // run(String::from(code));
    println!("{:#?}",code);
    Ok(())
}

fn run_prompt() -> Result<()> {
    print!("> ");
    let mut input_stream_reader = String::new();
    stdin().read_line(&mut input_stream_reader).expect("Enter Command");
    Ok(())
}
fn run(_bytes: String) {
    // unimplemented!()
}
