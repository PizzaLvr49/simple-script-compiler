use crate::lexer::Literal;
use crate::parser::{BinaryOp, Expression, Program, Statement};
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
            Value::String(s) => write!(f, "{s}"),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{0}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Value::Boolean(b) => write!(f, "{b}"),
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
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable '{name}'"),
            RuntimeError::UndefinedFunction(name) => write!(f, "Undefined function '{name}'"),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {msg}"),
            RuntimeError::ArityMismatch {
                function,
                expected,
                found,
            } => write!(
                f,
                "Function '{function}' expects {expected} arguments, but {found} were provided"
            ),
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

impl Default for Environment {
    fn default() -> Self {
        Self::new()
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
            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.evaluate_binary_op(left_val, op, right_val)
            }
        }
    }

    fn literal_to_value(&self, literal: Literal) -> Value {
        match literal {
            Literal::String(s) => Value::String(s),
            Literal::Number(n) => Value::Number(n),
            Literal::Boolean(b) => Value::Boolean(b),
        }
    }

    fn evaluate_binary_op(
        &self,
        left: Value,
        op: BinaryOp,
        right: Value,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                let result = match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Subtract => l - r,
                    BinaryOp::Multiply => l * r,
                    BinaryOp::Divide => {
                        if r == 0.0 {
                            return Err(RuntimeError::TypeError("Division by zero".to_string()));
                        }
                        l / r
                    }
                };
                Ok(Value::Number(result))
            }
            (Value::String(l), Value::String(r)) if matches!(op, BinaryOp::Add) => {
                Ok(Value::String(format!("{l}{r}")))
            }
            (l, r) => Err(RuntimeError::TypeError(format!(
                "Cannot apply {:?} to {} and {}",
                op,
                value_type_name(&l),
                value_type_name(&r)
            ))),
        }
    }

    fn call_function(
        &mut self,
        name: String,
        args: Vec<Expression>,
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
            print!("{arg}");
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

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn value_type_name(value: &Value) -> &str {
    match value {
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Boolean(_) => "boolean",
        Value::Null => "null",
    }
}
