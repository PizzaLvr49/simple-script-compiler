use simple_script_compiler::interpreter::{Interpreter, RuntimeError, Value};
use simple_script_compiler::lexer::{Lexer, Literal, Token};
use simple_script_compiler::parser::{BinaryOp, Expression, Parser, Statement};

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
fn lexer_basic_tokens_and_literals() {
    let src = r#"var x = 123; var s = "hello"; var f = 3.14; var b = true;"#;
    let mut lexer = Lexer::new(src);

    let kinds: Vec<&str> = std::iter::from_fn(|| {
        let t = lexer.current_token().clone();
        lexer.advance();
        Some(match t {
            Token::Var => "Var",
            Token::Identifier(_) => "Identifier",
            Token::Equals => "Equals",
            Token::SemiColon => "SemiColon",
            Token::Literal(_) => "Literal",
            Token::EOF => "EOF",
            other => panic!("Unexpected token in lexer test: {:?}", other),
        })
    })
    .take_while(|k| *k != "EOF")
    .collect();

    assert!(
        kinds
            .windows(3)
            .any(|w| w == ["Var", "Identifier", "Equals"])
    );
}

#[test]
fn parser_var_declaration_and_binary_expr() {
    let src = r#"var a = 1 + 2 * 3;"#;
    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("should parse");

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::VarDeclaration { name, value } => {
            assert_eq!(name, "a");
            match value {
                Expression::Binary { left, op, right } => {
                    assert_eq!(op, &BinaryOp::Add);
                    match **left {
                        Expression::Literal(Literal::Number(n)) => assert_eq!(n, 1.0),
                        ref other => panic!("expected left literal number, got {:?}", other),
                    }
                    match **right {
                        Expression::Binary {
                            left: ref inner_left,
                            op: ref inner_op,
                            right: ref inner_right,
                        } => {
                            assert_eq!(inner_op, &BinaryOp::Multiply);
                            match **inner_left {
                                Expression::Literal(Literal::Number(n)) => assert_eq!(n, 2.0),
                                ref other => {
                                    panic!("expected inner left literal number, got {:?}", other)
                                }
                            }
                            match **inner_right {
                                Expression::Literal(Literal::Number(n)) => assert_eq!(n, 3.0),
                                ref other => {
                                    panic!("expected inner right literal number, got {:?}", other)
                                }
                            }
                        }
                        ref other => panic!("expected multiply binary on right, got {:?}", other),
                    }
                }
                ref other => panic!("expected binary expression as var value, got {:?}", other),
            }
        }
        ref other => panic!("expected var declaration, got {:?}", other),
    }
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

    let num = |name: &str| match vars.get(name).expect("var exists") {
        Value::Number(n) => *n,
        other => panic!("{} is not a number: {:?}", name, other),
    };

    assert_eq!(num("sum"), 15.0);
    assert_eq!(num("product"), 50.0);
    assert_eq!(num("complex"), 14.0);
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

    match vars.get("greeting").unwrap() {
        Value::String(s) => assert_eq!(s, "Hello, World!"),
        other => panic!("greeting not string: {:?}", other),
    }

    match vars.get("ty").unwrap() {
        Value::String(s) => assert_eq!(s, "string"),
        other => panic!("ty not string: {:?}", other),
    }
}

#[test]
fn interpreter_negative_and_decimal() {
    let src = r#"
	var negative = -15;
	var calc = negative + 10;
	var pi = 3.14159;
	var radius = 5;
    var area = pi * radius * radius;
	"#;

    let interp = run_program(src).expect("should run");
    let vars = interp.get_variables();

    match vars.get("calc").unwrap() {
        Value::Number(n) => assert_eq!(*n, -5.0),
        other => panic!("calc not number: {:?}", other),
    }

    match vars.get("area").unwrap() {
        Value::Number(n) => assert!((*n - 3.14159 * 5.0 * 5.0).abs() < 1e-9),
        other => panic!("area not number: {:?}", other),
    }
}

#[test]
fn interpreter_runtime_errors_undefined_variable_and_type_error() {
    let src_undef = r#"var x = y + 1;"#;
    let lexer = Lexer::new(src_undef);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("parsed");
    let mut interp = Interpreter::new();

    let res = interp.interpret(program);
    assert!(res.is_err());
    match res.err().unwrap() {
        RuntimeError::UndefinedVariable(name) => assert_eq!(name, "y"),
        other => panic!("expected UndefinedVariable, got {:?}", other),
    }

    let src_type = r#"var a = "hi" + 1;"#;
    let lexer = Lexer::new(src_type);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("parsed");
    let mut interp = Interpreter::new();
    let res = interp.interpret(program);
    assert!(res.is_err());
    match res.err().unwrap() {
        RuntimeError::TypeError(_) => {}
        other => panic!("expected TypeError, got {:?}", other),
    }
}

#[test]
fn builtin_functions_print_and_arity() {
    let src = r#"
	var x = 1;
	println("value:", x);
	print("no newline");
	var y = 2;
	var sum = x + y;
	"#;

    let interp = run_program(src).expect("should run");
    let vars = interp.get_variables();
    match vars.get("sum").unwrap() {
        Value::Number(n) => assert_eq!(*n, 3.0),
        other => panic!("sum not number: {:?}", other),
    }

    let src_wrong = r#"var t = typeof();"#;
    let lexer = Lexer::new(src_wrong);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("parsed");
    let mut interp = Interpreter::new();
    let res = interp.interpret(program);
    assert!(res.is_err());
    match res.err().unwrap() {
        RuntimeError::ArityMismatch {
            function,
            expected,
            found,
        } => {
            assert_eq!(function, "typeof");
            assert_eq!(expected, 1);
            assert_eq!(found, 0);
        }
        other => panic!("expected ArityMismatch, got {:?}", other),
    }
}

#[test]
fn complex_program_runs_and_leaves_expected_state() {
    let src = r#"
	var myStr = "Hello, Rust!";
	var myNum = 4.28;
	var a = 10;
	var b = 5;
	var sum = a + b;
    var prod = a * b;
    var nested = (2 + 3) * 4;
	var concat = "Hi " + "there";
	var neg = -15;
	var calcNeg = neg + 10;
	"#;

    let interp = run_program(src).expect("should run");
    let vars = interp.get_variables();

    assert_eq!(
        vars.get("myStr").unwrap(),
        &Value::String("Hello, Rust!".to_string())
    );
    assert_eq!(vars.get("myNum").unwrap(), &Value::Number(4.28));
    assert_eq!(vars.get("sum").unwrap(), &Value::Number(15.0));
    assert_eq!(vars.get("prod").unwrap(), &Value::Number(50.0));
    assert_eq!(vars.get("nested").unwrap(), &Value::Number(20.0));
    assert_eq!(
        vars.get("concat").unwrap(),
        &Value::String("Hi there".to_string())
    );
    assert_eq!(vars.get("calcNeg").unwrap(), &Value::Number(-5.0));
}
