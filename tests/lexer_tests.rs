use simple_script_compiler::lexer::{Lexer, Literal, Token};

#[test]
fn lexer_basic_tokens_and_literals() {
    let src = r#"var x = 123; var s = "hello"; var f = 3.14; var b = true;"#;
    let mut lexer = Lexer::new(src);

    let kinds: Vec<&str> = std::iter::from_fn(|| {
        let t = lexer.current_token().clone();
        lexer.advance();
        Some(match t {
            Token::Var => "Var",
            Token::Identifier(_) => "Identifier",
            Token::Equals => "Equals",
            Token::SemiColon => "SemiColon",
            Token::Literal(_) => "Literal",
            Token::EOF => "EOF",
            other => panic!("Unexpected token in lexer test: {:?}", other),
        })
    })
    .take_while(|k| *k != "EOF")
    .collect();

    assert!(
        kinds
            .windows(3)
            .any(|w| w == ["Var", "Identifier", "Equals"])
    );
}

#[test]
fn lexer_number_edge_cases_and_negative_literals() {
    let src = r#"var a = -0.5; var b = .25; var c = 10.; var d = -123;"#;
    let mut lexer = Lexer::new(src);

    let mut numbers = Vec::new();
    loop {
        match lexer.current_token() {
            Token::Literal(Literal::Number(n)) => numbers.push(*n),
            Token::EOF => break,
            _ => {}
        }
        lexer.advance();
    }

    assert_eq!(numbers.len(), 4);
    assert_eq!(numbers[0], -0.5);
    assert_eq!(numbers[1], 0.25);
    assert_eq!(numbers[2], 10.0);
    assert_eq!(numbers[3], -123.0);
}
