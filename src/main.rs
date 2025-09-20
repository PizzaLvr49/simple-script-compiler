pub mod lexer;
pub mod parser;

fn main() {
    let source = "var myStr = \"Hello, Rust!\"; print(myStr); var myNum = 4.28; print(myNum); var test = myNum; print(test);";

    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            println!("Parsed program successfully!");
            println!("Number of statements: {}", program.statements.len());
            println!();

            for (i, stmt) in program.statements.iter().enumerate() {
                println!("Statement {}: {:?}", i + 1, stmt);

                match stmt {
                    parser::Statement::VarDeclaration { name, value } => {
                        println!("  Variable declaration: '{}' = {:?}", name, value);
                    }
                    parser::Statement::Expression(expr) => match expr {
                        parser::Expression::FunctionCall { name, args } => {
                            println!("  Function call: '{}' with {} arguments", name, args.len());
                            for (j, arg) in args.iter().enumerate() {
                                println!("    Arg {}: {:?}", j + 1, arg);
                            }
                        }
                        _ => {
                            println!("  Expression: {:?}", expr);
                        }
                    },
                }
                println!();
            }
        }
        Err(error) => {
            println!("Parse error: {:?}", error);
        }
    }
}
