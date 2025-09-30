pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let source = r#"
    var myStr = "Hello, Rust!";
    println(myStr);
    
    var myNum = 4.28;
    println("My number:", myNum);
    println("Type of myNum:", typeof(myNum));
    
    var a = 10;
    var b = 5;
    
    println("\n--- Arithmetic Operations ---");
    var sum = a + b;
    println("a + b =", sum);
    
    var difference = a - b;
    println("a - b =", difference);
    
    var product = a * b;
    println("a x b =", product);
    
    var quotient = a / b;
    println("a / b =", quotient);
    
    println("\n--- Operator Precedence ---");
    var result1 = 2 + 3 * 4;
    println("2 + 3 x 4 =", result1);
    
    var result2 = (2 + 3) * 4;
    println("(2 + 3) x 4 =", result2);
    
    var result3 = 10 + 5 * 2 - 8 / 4;
    println("10 + 5 x 2 - 8 / 4 =", result3);
    
    println("\n--- Complex Expressions ---");
    var complex = ((5 + 3) * 2) - 1;
    println("((5 + 3) x 2) - 1 =", complex);
    
    var withVars = a + b * 2;
    println("a + b x 2 =", withVars);
    
    println("\n--- String Concatenation ---");
    var greeting = "Hello, " + "World!";
    println(greeting);
    
    var name = "Alice";
    var message = "Welcome, " + name;
    println(message);
    
    println("\n--- Negative Numbers ---");
    var negative = -15;
    var calcNeg = negative + 10;
    println("-15 + 10 =", calcNeg);
    
    println("\n--- Decimal Arithmetic ---");
    var pi = 3.14159;
    var radius = 5;
    var area = pi * radius * radius;
    println("Area of circle (r=5):", area);
    "#;

    println!("=== Running Interpreter ===\n");

    let lexer = lexer::Lexer::new(source);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            let mut interpreter = interpreter::Interpreter::new();

            if let Err(runtime_error) = interpreter.interpret(program) {
                println!("\n❌ Runtime error: {}", runtime_error);
            } else {
                println!("\n✓ Program executed successfully!");
            }
        }
        Err(parse_error) => {
            println!("❌ Parse error: {:?}", parse_error);
        }
    }
}
