pub mod lexer;

fn main() {
    let source = "var myNum = 4; print(x);";
    let mut lexer = lexer::Lexer::new(source);

    while lexer.current_token() != &lexer::Token::EOF {
        println!("{:?}", lexer.current_token());
        lexer.advance();
    }
}