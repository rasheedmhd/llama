use std::fs::File;
use std::io::prelude::*;
use std::{env, io};
use std::process;

#[allow(dead_code)]

// abstract class Expr {
    // static class Binary extends Expr {
    // Binary(Expr left, Token operator, Expr right) {
        // this.left = left;
        // this.operator = operator;
        // this.right = right;
// } 
    // final Expr left;
    // final Token operator;
    // final Expr right;
    // } // Other expressions...
// }


pub struct GenerateAst {}

impl GenerateAst {
    #[allow(dead_code)]
    fn generate_ast() {
        let args: Vec<String> = env::args().collect();
    
        if args.len() != 1 {
            eprintln!("Usage: generate_ast <output directory>");
            process::exit(64);
        }
        let output_dir = &args[1];
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
        file.write_all(b"struct {base_name} {").unwrap();

        for t in types {
            let parts: Vec<&str> = t.split(":").collect();
            let class_name = parts[0].trim();
            let fields = parts[1].trim();
            println!("{:} {:}", class_name, fields);    
            Self::define_type(writer, base_name, class_name, fields);
        }
        file.write(b"}").unwrap();
    }
    fn define_type(writer: &mut dyn Write, base_name: &str, class_name: &str, fields: &str) -> io::Result<()> {
        // Write the struct definition
        writeln!(writer, "struct {} {{", struct_name)?;
        let fields: Vec<&str> = field_list.split(", ").collect();
        for field in &fields {
            writeln!(writer, "    pub {},", field)?;
        }
        writeln!(writer, "}}")?;

        // Write the impl block
        writeln!(writer, "impl {} {{", struct_name)?;
        writeln!(writer, "    pub fn new({}) -> Self {{", field_list)?;
        writeln!(writer, "        Self {{")?;
        for field in &fields {
            let name = field.split_whitespace().nth(1).unwrap();
            writeln!(writer, "            {},", name)?;
        }
        writeln!(writer, "        }}")?;
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;

        Ok(())
    }
}
