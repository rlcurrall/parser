use std::env;
use std::fs;
use tusk_lexer::Lexer;
use tusk_parser::Parser;

fn main() {
    let filepath = match env::args().nth(1) {
        Some(filepath) => filepath,
        None => {
            println!("Please provide a filepath to generate an AST.");
            std::process::exit(1);
        }
    };

    let contents = fs::read_to_string(filepath).unwrap();

    let lexer = Lexer::new(contents.as_str());
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    if let Err(error) = program {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    println!("{:?}", program.unwrap());
}
