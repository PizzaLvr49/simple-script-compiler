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
                                    continue;
                                }
                            }
                        }
                    }
                    self.chars.next();
                    continue;
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
                Some(ch) if ch.is_ascii_digit() => match self.read_number() {
                    Ok(num) => {
                        return Token::Literal(Literal::Number(num));
                    }
                    Err(_) => {
                        self.chars.next();
                        continue;
                    }
                },
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

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_tokens(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        loop {
            tokens.push(lexer.current_token().clone());
            if matches!(lexer.current_token(), Token::EOF) {
                break;
            }
            lexer.advance();
        }

        tokens
    }

    #[test]
    fn test_empty_input() {
        let tokens = collect_tokens("");
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = collect_tokens("   \t\n  ");
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_single_tokens() {
        let tokens = collect_tokens("= ; ( ) ,");
        assert_eq!(
            tokens,
            vec![
                Token::Equals,
                Token::SemiColon,
                Token::LeftParen,
                Token::RightParen,
                Token::Comma,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_var_keyword() {
        let tokens = collect_tokens("var");
        assert_eq!(tokens, vec![Token::Var, Token::EOF]);
    }

    #[test]
    fn test_boolean_literals() {
        let tokens = collect_tokens("true false");
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::Boolean(true)),
                Token::Literal(Literal::Boolean(false)),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_string_literals() {
        let tokens = collect_tokens(r#""hello" "world" """#);
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::String("hello".to_string())),
                Token::Literal(Literal::String("world".to_string())),
                Token::Literal(Literal::String("".to_string())),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_string_with_spaces() {
        let tokens = collect_tokens(r#""hello world""#);
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::String("hello world".to_string())),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_number_literals() {
        let tokens = collect_tokens("42 3.14 0 123.456");
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::Number(42.0)),
                Token::Literal(Literal::Number(3.14)),
                Token::Literal(Literal::Number(0.0)),
                Token::Literal(Literal::Number(123.456)),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_negative_numbers() {
        let tokens = collect_tokens("-42 -3.14");
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::Number(-42.0)),
                Token::Literal(Literal::Number(-3.14)),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = collect_tokens("myVar x test123 _private");
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("myVar".to_string()),
                Token::Identifier("x".to_string()),
                Token::Identifier("test123".to_string()),
                Token::Identifier("_private".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_variable_declaration() {
        let tokens = collect_tokens("var x = 42;");
        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Literal(Literal::Number(42.0)),
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_string_variable_declaration() {
        let tokens = collect_tokens(r#"var greeting = "Hello, World!";"#);
        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Identifier("greeting".to_string()),
                Token::Equals,
                Token::Literal(Literal::String("Hello, World!".to_string())),
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_boolean_variable_declaration() {
        let tokens = collect_tokens("var flag = true;");
        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Identifier("flag".to_string()),
                Token::Equals,
                Token::Literal(Literal::Boolean(true)),
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = collect_tokens("print()");
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("print".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_function_call_with_args() {
        let tokens = collect_tokens(r#"print("hello", 42, true)"#);
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("print".to_string()),
                Token::LeftParen,
                Token::Literal(Literal::String("hello".to_string())),
                Token::Comma,
                Token::Literal(Literal::Number(42.0)),
                Token::Comma,
                Token::Literal(Literal::Boolean(true)),
                Token::RightParen,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_complex_program() {
        let tokens = collect_tokens(
            r#"
            var name = "Alice";
            var age = 25;
            var isActive = true;
            print(name, age, isActive);
        "#,
        );

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Identifier("name".to_string()),
                Token::Equals,
                Token::Literal(Literal::String("Alice".to_string())),
                Token::SemiColon,
                Token::Var,
                Token::Identifier("age".to_string()),
                Token::Equals,
                Token::Literal(Literal::Number(25.0)),
                Token::SemiColon,
                Token::Var,
                Token::Identifier("isActive".to_string()),
                Token::Equals,
                Token::Literal(Literal::Boolean(true)),
                Token::SemiColon,
                Token::Identifier("print".to_string()),
                Token::LeftParen,
                Token::Identifier("name".to_string()),
                Token::Comma,
                Token::Identifier("age".to_string()),
                Token::Comma,
                Token::Identifier("isActive".to_string()),
                Token::RightParen,
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_advance_and_current_token() {
        let mut lexer = Lexer::new("var x = 42;");

        assert_eq!(lexer.current_token(), &Token::Var);
        lexer.advance();
        assert_eq!(lexer.current_token(), &Token::Identifier("x".to_string()));
        lexer.advance();
        assert_eq!(lexer.current_token(), &Token::Equals);
        lexer.advance();
        assert_eq!(
            lexer.current_token(),
            &Token::Literal(Literal::Number(42.0))
        );
        lexer.advance();
        assert_eq!(lexer.current_token(), &Token::SemiColon);
        lexer.advance();
        assert_eq!(lexer.current_token(), &Token::EOF);
        lexer.advance();
        assert_eq!(lexer.current_token(), &Token::EOF);
    }

    #[test]
    fn test_unterminated_string() {
        let tokens = collect_tokens(r#""unterminated"#);
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::String("unterminated".to_string())),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_invalid_characters_skipped() {
        let tokens = collect_tokens("var @ x # = $ 42 % ;");
        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Literal(Literal::Number(42.0)),
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_zero_number() {
        let tokens = collect_tokens("0");
        assert_eq!(
            tokens,
            vec![Token::Literal(Literal::Number(0.0)), Token::EOF]
        );
    }

    #[test]
    fn test_decimal_numbers() {
        let tokens = collect_tokens("0.5 .5 5.");
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::Number(0.5)),
                Token::Literal(Literal::Number(0.5)),
                Token::Literal(Literal::Number(5.0)),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_consecutive_tokens() {
        let tokens = collect_tokens("var()x=42;");
        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::LeftParen,
                Token::RightParen,
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Literal(Literal::Number(42.0)),
                Token::SemiColon,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_mixed_case_identifiers() {
        let tokens = collect_tokens("MyVar myVar MYVAR");
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("MyVar".to_string()),
                Token::Identifier("myVar".to_string()),
                Token::Identifier("MYVAR".to_string()),
                Token::EOF
            ]
        );
    }
}
