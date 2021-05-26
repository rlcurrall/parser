use tusk_lexer::Lexer;
use tusk_parser::Parser;
use tusk_parser::Statement;
use tusk_parser::Expression;

#[test]
fn test_it_can_be_created() {
    let mut lexer = Lexer::new("");

    Parser::new(lexer);
}

#[test]
fn test_it_can_parse_literals() {
    assert_statements_match("'Hello, world!' 12345 6789.1234", vec![
        Statement::Expression(Expression::String("Hello, world!".to_owned())),
        Statement::Expression(Expression::Integer(12345)),
        Statement::Expression(Expression::Float(6789.1234))
    ]);
}

fn assert_statements_match(source: &str, statements: Vec<Statement>) {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    assert_eq!(program.unwrap(), statements);
}