use crate::lexer::{Lexer, Literal, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration { name: String, value: Expression },
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEOF,
    InvalidExpression,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !matches!(self.lexer.current_token(), Token::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.lexer.current_token() {
            Token::Var => self.parse_var_declaration(),
            _ => {
                let expr = self.parse_expression()?;
                self.expect_token(Token::SemiColon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement, ParseError> {
        self.lexer.advance();

        let name = match self.lexer.current_token() {
            Token::Identifier(name) => name.clone(),
            token => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: token.clone(),
                });
            }
        };
        self.lexer.advance();

        self.expect_token(Token::Equals)?;

        let value = self.parse_expression()?;

        self.expect_token(Token::SemiColon)?;

        Ok(Statement::VarDeclaration { name, value })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.lexer.current_token() {
            Token::Literal(literal) => {
                let expr = Expression::Literal(literal.clone());
                self.lexer.advance();
                Ok(expr)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.lexer.advance();

                if matches!(self.lexer.current_token(), Token::LeftParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            token => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token.clone(),
            }),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<Expression, ParseError> {
        self.expect_token(Token::LeftParen)?;

        let mut args = Vec::new();

        if matches!(self.lexer.current_token(), Token::RightParen) {
            self.lexer.advance();
            return Ok(Expression::FunctionCall { name, args });
        }

        loop {
            let arg = self.parse_expression()?;
            args.push(arg);

            match self.lexer.current_token() {
                Token::Comma => {
                    self.lexer.advance();
                    continue;
                }
                Token::RightParen => {
                    self.lexer.advance();
                    break;
                }
                token => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "',' or ')'".to_string(),
                        found: token.clone(),
                    });
                }
            }
        }

        Ok(Expression::FunctionCall { name, args })
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if std::mem::discriminant(self.lexer.current_token()) == std::mem::discriminant(&expected) {
            self.lexer.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: self.lexer.current_token().clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Lexer, Literal};

    #[test]
    fn test_var_declaration() {
        let input = "var x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(value, &Expression::Literal(Literal::Number(42.0)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_string_var_declaration() {
        let input = r#"var greeting = "hello";"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "greeting");
                assert_eq!(
                    value,
                    &Expression::Literal(Literal::String("hello".to_string()))
                );
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_boolean_var_declaration() {
        let input = "var flag = true;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "flag");
                assert_eq!(value, &Expression::Literal(Literal::Boolean(true)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_false_boolean_var_declaration() {
        let input = "var flag = false;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "flag");
                assert_eq!(value, &Expression::Literal(Literal::Boolean(false)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_negative_number_var_declaration() {
        let input = "var x = -42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(value, &Expression::Literal(Literal::Number(-42.0)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_decimal_var_declaration() {
        let input = "var pi = 3.14;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "pi");
                assert_eq!(value, &Expression::Literal(Literal::Number(3.14)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_function_call() {
        let input = "print();";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_function_call_with_one_arg() {
        let input = r#"print("hello");"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 1);
                assert_eq!(
                    args[0],
                    Expression::Literal(Literal::String("hello".to_string()))
                );
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_function_call_with_args() {
        let input = r#"print("hello", 42, true);"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 3);
                assert_eq!(
                    args[0],
                    Expression::Literal(Literal::String("hello".to_string()))
                );
                assert_eq!(args[1], Expression::Literal(Literal::Number(42.0)));
                assert_eq!(args[2], Expression::Literal(Literal::Boolean(true)));
            }
            _ => panic!("Expected function call with arguments"),
        }
    }

    #[test]
    fn test_function_call_with_identifier_args() {
        let input = "print(x, y);";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], Expression::Identifier("x".to_string()));
                assert_eq!(args[1], Expression::Identifier("y".to_string()));
            }
            _ => panic!("Expected function call with identifier arguments"),
        }
    }

    #[test]
    fn test_multiple_statements() {
        let input = r#"
            var x = 10;
            var y = "test";
            print(x, y);
        "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 3);

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(value, &Expression::Literal(Literal::Number(10.0)));
            }
            _ => panic!("Expected first statement to be variable declaration"),
        }

        match &program.statements[1] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "y");
                assert_eq!(
                    value,
                    &Expression::Literal(Literal::String("test".to_string()))
                );
            }
            _ => panic!("Expected second statement to be variable declaration"),
        }

        match &program.statements[2] {
            Statement::Expression(Expression::FunctionCall { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected third statement to be function call"),
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "var y = x;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "y");
                assert_eq!(value, &Expression::Identifier("x".to_string()));
            }
            _ => panic!("Expected variable declaration with identifier"),
        }
    }

    #[test]
    fn test_underscore_identifier() {
        let input = "var _private = 123;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "_private");
                assert_eq!(value, &Expression::Literal(Literal::Number(123.0)));
            }
            _ => panic!("Expected variable declaration with underscore identifier"),
        }
    }

    #[test]
    fn test_complex_program() {
        let input = r#"
            var name = "Alice";
            var age = 25;
            var score = -10.5;
            var isActive = true;
            var _temp = false;
            print(name);
            calculate(age, score);
            log();
        "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 8);

        let expected_vars = vec![
            (
                "name",
                Expression::Literal(Literal::String("Alice".to_string())),
            ),
            ("age", Expression::Literal(Literal::Number(25.0))),
            ("score", Expression::Literal(Literal::Number(-10.5))),
            ("isActive", Expression::Literal(Literal::Boolean(true))),
            ("_temp", Expression::Literal(Literal::Boolean(false))),
        ];

        for (i, (expected_name, expected_value)) in expected_vars.iter().enumerate() {
            match &program.statements[i] {
                Statement::VarDeclaration { name, value } => {
                    assert_eq!(name, expected_name);
                    assert_eq!(value, expected_value);
                }
                _ => panic!("Expected variable declaration at index {}", i),
            }
        }

        let expected_calls = vec![("print", 1), ("calculate", 2), ("log", 0)];

        for (i, (expected_name, expected_arg_count)) in expected_calls.iter().enumerate() {
            let stmt_index = i + 5;
            match &program.statements[stmt_index] {
                Statement::Expression(Expression::FunctionCall { name, args }) => {
                    assert_eq!(name, expected_name);
                    assert_eq!(args.len(), *expected_arg_count);
                }
                _ => panic!("Expected function call at index {}", stmt_index),
            }
        }
    }

    #[test]
    fn test_nested_function_call() {
        let input = "var result = getValue();";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "result");
                match value {
                    Expression::FunctionCall { name, args } => {
                        assert_eq!(name, "getValue");
                        assert_eq!(args.len(), 0);
                    }
                    _ => panic!("Expected function call as value"),
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_empty_program() {
        let input = "";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_whitespace_only_program() {
        let input = "   \n\t  \n  ";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_decimal_starting_with_dot() {
        let input = "var half = .5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "half");
                assert_eq!(value, &Expression::Literal(Literal::Number(0.5)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_missing_semicolon_error() {
        let input = "var x = 42";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found: _ } => {
                assert!(expected.contains("SemiColon"));
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_missing_equals_error() {
        let input = "var x 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found: _ } => {
                assert!(expected.contains("Equals"));
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_missing_identifier_error() {
        let input = "var = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found: _ } => {
                assert!(expected.contains("identifier"));
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_unclosed_function_call_error() {
        let input = "print(hello;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found: _ } => {
                assert!(expected.contains("')'"));
            }
            _ => panic!("Expected UnexpectedToken error for missing closing paren"),
        }
    }

    #[test]
    fn test_invalid_expression_start() {
        let input = "var x = );";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found: _ } => {
                assert!(expected.contains("expression"));
            }
            _ => panic!("Expected UnexpectedToken error for invalid expression"),
        }
    }
}
