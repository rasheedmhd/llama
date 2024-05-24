use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::iter::Enumerate;
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

    define_visitor(&mut file, base_name, &types)?;

    for type_def in &types {
        let enum_name = type_def.split(':').next().unwrap().trim();
        let fields = type_def.split(':').nth(1).unwrap().trim();
        define_type(&mut file, base_name, enum_name, fields)?;
    }

    writeln!(file, "}}")?;

    Ok(())
}


fn define_type<W: Write>(writer: &mut W, base_name: &str, struct_name: &str, field_list: &str) -> io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "    pub struct {}{} {{", struct_name, base_name)?;

    for field in field_list.split(", ") {
        writeln!(writer, "        pub {},", field)?;
    }

    writeln!(writer, "    }}")?;
    writeln!(writer)?;

    writeln!(writer, "    impl {}{} {{", struct_name, base_name)?;
    writeln!(writer, "        pub fn new({}) -> Self {{", field_list)?;

    writeln!(writer, "            Self {{")?;
    for field in field_list.split(", ") {
        let name = field.split(' ').nth(1).unwrap();
        writeln!(writer, "                {},", name)?;
    }
    writeln!(writer, "            }}")?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer)?;

    writeln!(writer, "    impl {} for {}{} {{", base_name, struct_name, base_name)?;
    writeln!(writer, "        fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {{")?;
    writeln!(writer, "            visitor.visit_{}_{}(self)", struct_name.to_lowercase(), base_name.to_lowercase())?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer)?;

    Ok(())
}

fn define_visitor<W: Write>(writer: &mut W, base_name: &str, types: &[&str]) -> io::Result<()> {
    writeln!(writer, "    pub trait Visitor<T> {{")?;

    for type_def in types {
        let type_name = type_def.split(':').next().unwrap().trim();
        writeln!(writer, "        fn visit_{}_{}(&mut self, {}: &{}{}) -> T;",
                 type_name.to_lowercase(), base_name.to_lowercase(),
                 base_name.to_lowercase(), type_name, base_name)?;
    }

    writeln!(writer, "    }}")?;
    writeln!(writer)?;

    Ok(())
}