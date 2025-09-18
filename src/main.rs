pub mod lexer;

fn main() {
    let source = "var myNum = 4; print(x);";
    let mut lexer = lexer::Lexer::new(source);

    while lexer.current_token() != &lexer::Token::EOF {
        println!("{:?}", lexer.current_token());
        lexer.advance();
    }
    
    println!("{:?}", lexer.current_token());
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test() {
        use crate::lexer::{Token, Constant};
        
        let source = "var myNum = 4; print(x);";
        let mut lexer = crate::lexer::Lexer::new(source);
        
        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("myNum".to_string()),
            Token::Equals, 
            Token::Constant(Constant::Number(4)),
            Token::SemiColon,
            Token::Identifier("print".to_string()),
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen, 
            Token::SemiColon,
            Token::EOF
        ];
        
        let mut actual_tokens = Vec::new();
        
        while lexer.current_token() != &Token::EOF {
            actual_tokens.push(lexer.current_token().clone());
            lexer.advance();
        }
        actual_tokens.push(lexer.current_token().clone());
        
        assert_eq!(actual_tokens, expected_tokens);
    }
}