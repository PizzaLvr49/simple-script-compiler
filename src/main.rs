use simple_script_compiler::{Interpreter, Lexer, Parser};

fn main() {
    let source = r#"
    var x = 32424 * 312;
    println(2 * 4 + (3 + 6 + -7 * 2) / 2 + x - -3 * (x * x * x * x * x));
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            let mut interpreter = Interpreter::new();

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
