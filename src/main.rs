pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let source = "var myStr = \"Hello, Rust!\"; println(myStr); var myNum = 4.28; println(myNum); println(\"Type of myNum:\", typeof(myNum));";

    println!("Source code:");
    println!("{}", source);
    println!("\n{}", "=".repeat(50));
    println!("Execution output:");

    // Parse the source code
    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            // Create and run the interpreter
            let mut interpreter = interpreter::Interpreter::new();

            match interpreter.interpret(program) {
                Ok(()) => {
                    println!("\n{}", "=".repeat(50));
                    println!("Program executed successfully!");

                    // Show final variable state
                    println!("\nFinal variables:");
                    for (name, value) in interpreter.get_variables() {
                        println!(
                            "  {} = {} ({})",
                            name,
                            value,
                            match value {
                                interpreter::Value::String(_) => "string",
                                interpreter::Value::Number(_) => "number",
                                interpreter::Value::Boolean(_) => "boolean",
                                interpreter::Value::Null => "null",
                            }
                        );
                    }
                }
                Err(runtime_error) => {
                    println!("\nRuntime error: {}", runtime_error);
                }
            }
        }
        Err(parse_error) => {
            println!("Parse error: {:?}", parse_error);
        }
    }
}
