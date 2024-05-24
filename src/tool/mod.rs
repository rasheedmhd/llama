use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = &args[1];
    define_ast(output_dir, "Expr", vec![
        "Binary   : Box<Expr> left, Token operator, Box<Expr> right",
        "Grouping : Box<Expr> expression",
        "Literal  : LiteralValue value",
        "Unary    : Token operator, Box<Expr> right",
    ])
}


fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let path = Path::new(output_dir).join(format!("{}.rs", base_name.to_lowercase()));
    let mut file = File::create(&path)?;

    writeln!(file, "pub mod llama_ast {{")?;
    writeln!(file)?;
    writeln!(file, "    use super::token::Token;")?;
    writeln!(file)?;

    writeln!(file, "    pub enum {} {{", base_name)?;

    for type_def in &types {
        let enum_name = type_def.split(':').next().unwrap().trim();
        writeln!(file, "        {}({}{}),", enum_name, enum_name, base_name)?;
    }

    writeln!(file, "    }}")?;
    writeln!(file)?;

    // define_visitor(&mut file, base_name, &types)?;

    for type_def in &types {
        let enum_name = type_def.split(':').next().unwrap().trim();
        let fields = type_def.split(':').nth(1).unwrap().trim();
        define_type(&mut file, base_name, enum_name, fields)?;
    }

    writeln!(file, "}}")?;

    Ok(())
}
