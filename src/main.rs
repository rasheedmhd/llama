use llama::repl::Llama;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;
use std::process;

fn main() {
    // create_keywords();
    println!("Llama - A programming language impl from Robert Nystrom's Crafting Interpreters");
    Llama::start();
}


pub struct GenerateAst {}

impl GenerateAst {
    fn generate_ast() {
        let args: Vec<String> = env::args().collect();
    
        if args.len() != 1 {
            eprintln!("Usage: generate_ast <output directory>");
            process::exit(64);
        }
        let output_dir = &args[0];
        GenerateAst::define_ast(output_dir, "Expr",
            vec![
                "Binary : Expr left, Token operator, Expr right",
                "Grouping : Expr expression",
                "Literal : Object value",
                "Unary : Token operator, Expr right"
            ]
        );
    }

    fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) {
        let path = format!("{}/{}.rs", output_dir, base_name);
        let mut file = File::create(&path).unwrap();
        let content = b"struct {base_name} {

        }";
        file.write_all(content).unwrap();
    }
}
