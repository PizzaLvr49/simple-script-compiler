use crate::lexer::Literal;
use crate::parser::{ Expression, Program, Statement };
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => {
                if n.fract() == 0.0 { write!(f, "{}", *n as i64) } else { write!(f, "{}", n) }
            }
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeError(String),
    ArityMismatch {
        function: String,
        expected: usize,
        found: usize,
    },
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => { write!(f, "Undefined variable '{}'", name) }
            RuntimeError::UndefinedFunction(name) => { write!(f, "Undefined function '{}'", name) }
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::ArityMismatch { function, expected, found } => {
                write!(
                    f,
                    "Function '{}' expects {} arguments, but {} were provided",
                    function,
                    expected,
                    found
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))
    }
}

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        for statement in program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::VarDeclaration { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define(name, val);
                Ok(())
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(literal) => Ok(self.literal_to_value(literal)),
            Expression::Identifier(name) => self.environment.get(&name),
            Expression::FunctionCall { name, args } => self.call_function(name, args),
        }
    }

    fn literal_to_value(&self, literal: Literal) -> Value {
        match literal {
            Literal::String(s) => Value::String(s),
            Literal::Number(n) => Value::Number(n),
            Literal::Boolean(b) => Value::Boolean(b),
        }
    }

    fn call_function(
        &mut self,
        name: String,
        args: Vec<Expression>
    ) -> Result<Value, RuntimeError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate_expression(arg)?);
        }

        match name.as_str() {
            "print" => self.builtin_print(arg_values),
            "println" => self.builtin_println(arg_values),
            "typeof" => self.builtin_typeof(arg_values),
            _ => Err(RuntimeError::UndefinedFunction(name)),
        }
    }

    fn builtin_print(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        Ok(Value::Null)
    }

    fn builtin_println(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        self.builtin_print(args)?;
        println!();
        Ok(Value::Null)
    }

    fn builtin_typeof(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError::ArityMismatch {
                function: "typeof".to_string(),
                expected: 1,
                found: args.len(),
            });
        }

        let type_name = match &args[0] {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
        };

        Ok(Value::String(type_name.to_string()))
    }

    pub fn get_variables(&self) -> &HashMap<String, Value> {
        &self.environment.variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn interpret_source(source: &str) -> Result<Interpreter, RuntimeError> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse().expect("Failed to parse");

        let mut interpreter = Interpreter::new();
        interpreter.interpret(program)?;
        Ok(interpreter)
    }

    #[test]
    fn test_variable_declaration_and_retrieval() {
        let interpreter = interpret_source("var x = 42;").unwrap();
        assert_eq!(interpreter.get_variables().get("x"), Some(&Value::Number(42.0)));
    }

    #[test]
    fn test_string_variable() {
        let interpreter = interpret_source(r#"var greeting = "hello";"#).unwrap();
        assert_eq!(
            interpreter.get_variables().get("greeting"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_boolean_variable() {
        let interpreter = interpret_source("var flag = true;").unwrap();
        assert_eq!(interpreter.get_variables().get("flag"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_multiple_variables() {
        let source =
            r#"
            var name = "Alice";
            var age = 25;
            var active = true;
        "#;
        let interpreter = interpret_source(source).unwrap();

        assert_eq!(
            interpreter.get_variables().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(interpreter.get_variables().get("age"), Some(&Value::Number(25.0)));
        assert_eq!(interpreter.get_variables().get("active"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn test_variable_reference() {
        let source = r#"
            var x = 10;
            var y = x;
        "#;
        let interpreter = interpret_source(source).unwrap();

        assert_eq!(interpreter.get_variables().get("x"), Some(&Value::Number(10.0)));
        assert_eq!(interpreter.get_variables().get("y"), Some(&Value::Number(10.0)));
    }

    #[test]
    fn test_undefined_variable_error() {
        let result = interpret_source("var y = x;");
        assert!(result.is_err());
        match result.unwrap_err() {
            RuntimeError::UndefinedVariable(name) => assert_eq!(name, "x"),
            _ => panic!("Expected UndefinedVariable error"),
        }
    }

    #[test]
    fn test_print_function() {
        let result = interpret_source(r#"print("Hello, World!");"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_with_variables() {
        let source =
            r#"
            var name = "Alice";
            var age = 25;
            print(name, age);
        "#;
        let result = interpret_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_typeof_function() {
        let source = r#"
            var x = 42;
            var t = typeof(x);
        "#;
        let interpreter = interpret_source(source).unwrap();
        assert_eq!(
            interpreter.get_variables().get("t"),
            Some(&Value::String("number".to_string()))
        );
    }

    #[test]
    fn test_undefined_function_error() {
        let result = interpret_source("unknown_function();");
        assert!(result.is_err());
        match result.unwrap_err() {
            RuntimeError::UndefinedFunction(name) => assert_eq!(name, "unknown_function"),
            _ => panic!("Expected UndefinedFunction error"),
        }
    }

    #[test]
    fn test_arity_mismatch_error() {
        let result = interpret_source("typeof();");
        assert!(result.is_err());
        match result.unwrap_err() {
            RuntimeError::ArityMismatch { function, expected, found } => {
                assert_eq!(function, "typeof");
                assert_eq!(expected, 1);
                assert_eq!(found, 0);
            }
            _ => panic!("Expected ArityMismatch error"),
        }
    }
}
