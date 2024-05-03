#[allow(unused_variables)]
use std::env;
use std::fs;
use std::io::stdin;
use std::process;

fn main() {
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    get();
}

// Exit Codes
// https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
pub fn get() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("Usage: llama [script] {:?}", args.len());
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
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
    run(code);
}

fn run_prompt() {

    loop {
        println!(" > ");
        let mut input_stream_reader = String::new();
        stdin().read_line(&mut input_stream_reader).expect("Enter Command");
        run(input_stream_reader.clone());
    }
}

fn run(code: String) {
    println!("interpreting .... {:?}", code.trim());
}
