pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let source =
        "var myStr = \"Hello, Rust!\"; println(myStr); var myNum = 4.28; println(myNum); println(\"Type of myNum:\", typeof(myNum));";

    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            let mut interpreter = interpreter::Interpreter::new();

            if let Err(runtime_error) = interpreter.interpret(program) {
                println!("\nRuntime error: {}", runtime_error);
            }
        }
        Err(parse_error) => {
            println!("Parse error: {:?}", parse_error);
        }
    }
}
