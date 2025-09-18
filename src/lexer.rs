use std::{iter, num::ParseIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    String(String),
    Number(u32),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,
    Identifier(String),
    Equals,
    SemiColon,
    Constant(Constant),
    LeftParen,
    RightParen,
    Comma,
    EOF,
}

pub struct Lexer<'a> {
    chars: iter::Peekable<std::str::Chars<'a>>,
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

    fn read_number(&mut self) -> Result<u32, ParseIntError> {
        let mut num_str = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
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

    fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            match self.chars.peek() {
                None => return Token::EOF,
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
                Some(&ch) if ch.is_ascii_digit() => {
                    match self.read_number() {
                        Ok(num) => return Token::Constant(Constant::Number(num)),
                        Err(_) => {
                            self.chars.next();
                            continue;
                        }
                    }
                }
                Some(&ch) if ch.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "var" => return Token::Var,
                        "true" => return Token::Constant(Constant::Boolean(true)),
                        "false" => return Token::Constant(Constant::Boolean(false)),
                        _ => return Token::Identifier(identifier),
                    }
                }
                Some(_) => {
                    self.chars.next();
                    continue;
                }
            }
        }
    }
}