use std::{iter, num::ParseFloatError, str};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,
    Identifier(String),
    Equals,
    SemiColon,
    Literal(Literal),
    LeftParen,
    RightParen,
    Comma,
    Add,
    Subtract,
    Multiply,
    Divide,
    EOF,
}

pub struct Lexer<'a> {
    chars: iter::Peekable<str::Chars<'a>>,
    current_token: Token,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            chars: input.chars().peekable(),
            current_token: Token::EOF,
        };
        lexer.advance();
        lexer
    }

    pub fn current_token(&self) -> &Token {
        &self.current_token
    }

    pub fn advance(&mut self) {
        self.current_token = self.next_token();
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, ParseFloatError> {
        let mut num_str = String::new();

        if let Some(&'-') = self.chars.peek() {
            num_str.push('-');
            self.chars.next();
        }

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        num_str.parse()
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        identifier
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();

        if let Some(&ch) = self.chars.peek() {
            if ch != '"' {
                return string;
            }
            self.chars.next();
        } else {
            return string;
        }

        while let Some(&ch) = self.chars.peek() {
            self.chars.next();

            if ch == '"' {
                break;
            } else {
                string.push(ch);
            }
        }

        string
    }

    fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            match self.chars.peek() {
                None => {
                    return Token::EOF;
                }
                Some(&'=') => {
                    self.chars.next();
                    return Token::Equals;
                }
                Some(&';') => {
                    self.chars.next();
                    return Token::SemiColon;
                }
                Some(&'(') => {
                    self.chars.next();
                    return Token::LeftParen;
                }
                Some(&')') => {
                    self.chars.next();
                    return Token::RightParen;
                }
                Some(&',') => {
                    self.chars.next();
                    return Token::Comma;
                }
                Some(&'-') => {
                    let mut chars_clone = self.chars.clone();
                    chars_clone.next();
                    if let Some(&ch) = chars_clone.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            match self.read_number() {
                                Ok(num) => {
                                    return Token::Literal(Literal::Number(num));
                                }
                                Err(_) => {
                                    self.chars.next();
                                    return Token::Subtract;
                                }
                            }
                        }
                    }
                    self.chars.next();
                    return Token::Subtract;
                }
                Some(&'.') => {
                    let mut chars_clone = self.chars.clone();
                    chars_clone.next();
                    if let Some(&ch) = chars_clone.peek() {
                        if ch.is_ascii_digit() {
                            match self.read_number() {
                                Ok(num) => {
                                    return Token::Literal(Literal::Number(num));
                                }
                                Err(_) => {
                                    self.chars.next();
                                    continue;
                                }
                            }
                        }
                    }
                    self.chars.next();
                    continue;
                }
                Some(&'+') => {
                    self.chars.next();
                    return Token::Add;
                }
                Some(&'*') => {
                    self.chars.next();
                    return Token::Multiply;
                }
                Some(&'/') => {
                    self.chars.next();
                    return Token::Divide;
                }
                Some(ch) if ch.is_ascii_digit() => {
                    if let Ok(num) = self.read_number() {
                        return Token::Literal(Literal::Number(num));
                    } else {
                        self.chars.next();
                        continue;
                    }
                }
                Some(ch) if ch.is_alphabetic() || *ch == '_' => {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "var" => {
                            return Token::Var;
                        }
                        "true" => {
                            return Token::Literal(Literal::Boolean(true));
                        }
                        "false" => {
                            return Token::Literal(Literal::Boolean(false));
                        }
                        _ => {
                            return Token::Identifier(identifier);
                        }
                    }
                }
                Some(&'"') => {
                    let string = self.read_string();
                    return Token::Literal(Literal::String(string));
                }
                Some(_) => {
                    self.chars.next();
                    continue;
                }
            }
        }
    }
}
