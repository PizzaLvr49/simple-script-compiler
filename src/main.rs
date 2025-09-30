pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let source = r#"
    println(2 * 4 + (3 + 6 + -7 * 2) / 2);
    "#;

    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            let mut interpreter = interpreter::Interpreter::new();

            if let Err(runtime_error) = interpreter.interpret(program) {
                println!("Runtime error: {runtime_error}");
            } else {
                println!("Program executed successfully!");
            }
        }
        Err(parse_error) => {
            println!("Parse error: {parse_error:?}");
        }
    }
}
