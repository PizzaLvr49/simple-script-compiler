use crate::lexer::{Lexer, Literal, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
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
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.lexer.current_token() {
                Token::Add => BinaryOp::Add,
                Token::Subtract => BinaryOp::Subtract,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_multiplicative()?;

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;

        loop {
            let op = match self.lexer.current_token() {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_primary()?;

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
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
            Token::LeftParen => {
                self.lexer.advance();
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
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
                expected: format!("{expected:?}"),
                found: self.lexer.current_token().clone(),
            })
        }
    }
}
