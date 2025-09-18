pub mod lexer;

fn main() {
    let source = "var x = 42; var flag = true;";
    let mut lexer = lexer::Lexer::new(source);

    while lexer.current_token() != &lexer::Token::EOF {
        println!("{:?}", lexer.current_token());
        lexer.advance();
    }
}