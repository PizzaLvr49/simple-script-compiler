use simple_script_compiler::interpreter::{Interpreter, Value};
use simple_script_compiler::lexer::Lexer;
use simple_script_compiler::parser::Parser;

fn run_program(source: &str) -> Result<Interpreter, String> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut interpreter = Interpreter::new();
    interpreter
        .interpret(program)
        .map_err(|e| format!("Runtime error: {}", e))?;

    Ok(interpreter)
}

#[test]
fn interpreter_arithmetic_and_precedence() {
    let src = r#"
    var a = 10;
    var b = 5;
    var sum = a + b;
    var product = a * b;
    var complex = 2 + 3 * 4;
    "#;

    let interp = run_program(src).expect("should run");
    let vars = interp.get_variables();
    match vars.get("sum").unwrap() {
        Value::Number(n) => assert_eq!(*n, 15.0),
        _ => panic!("sum not number"),
    }
}

#[test]
fn interpreter_strings_and_concatenation_and_typeof() {
    let src = r#"
    var s = "Hello";
    var t = "World";
    var greeting = s + ", " + t + "!";
    var ty = typeof(greeting);
    "#;

    let interp = run_program(src).expect("should run");
    let vars = interp.get_variables();
    assert_eq!(
        vars.get("greeting").unwrap(),
        &Value::String("Hello, World!".to_string())
    );
    assert_eq!(
        vars.get("ty").unwrap(),
        &Value::String("string".to_string())
    );
}
