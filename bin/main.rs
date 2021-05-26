use std::env;
use std::fs;
use tusk_lexer::Lexer;
use tusk_parser::Parser;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filepath).unwrap();

    let lexer = Lexer::new(contents.as_str());
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    if let Err(error) = program {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}