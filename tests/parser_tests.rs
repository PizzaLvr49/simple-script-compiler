use simple_script_compiler::lexer::Lexer;
use simple_script_compiler::parser::{BinaryOp, Expression, Parser, Statement};
// ...existing code...

#[test]
fn parser_var_declaration_and_binary_expr() {
    let src = r#"var a = 1 + 2 * 3;"#;
    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("should parse");

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::VarDeclaration { name, value } => {
            assert_eq!(name, "a");
            match value {
                Expression::Binary {
                    left: _left,
                    op,
                    right: _right,
                } => {
                    assert_eq!(op, &BinaryOp::Add);
                }
                _ => panic!("expected binary expression"),
            }
        }
        _ => panic!("expected var declaration"),
    }
}

#[test]
fn parser_parentheses_and_nested_precedence() {
    let src = r#"var v = (1 + (2 + 3) * (4 + 5)) * 2;"#;
    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("should parse nested");
    assert!(!program.statements.is_empty());
}
