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
    fn generate_ast() {
        let args: Vec<String> = env::args().collect();
    
        if args.len() != 1 {
            eprintln!("Usage: generate_ast <output directory>");
            process::exit(64);
        }
        let output_dir = &args[0];
        define_ast(output_dir, "Expr",
            vec![
                "Binary : Expr left, Token operator, Expr right",
                "Grouping : Expr expression",
                "Literal : Object value",
                "Unary : Token operator, Expr right"
            ]
        );
    }

    fn define_ast(output_dir: String, base_name: String, types: Vec) {
        let path = format!("{}/{}.rs", output_dir,base_name);
    }
}
