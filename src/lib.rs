use tusk_lexer::Lexer;
use wasm_bindgen::prelude::*;

mod error;
mod expression;
mod function;
mod parser;
mod statement;

pub use error::ParserError;
pub use expression::Expression;
pub use function::Function;
pub use function::FunctionParameter;
pub use parser::Parser;
pub use statement::Statement;

#[wasm_bindgen]
#[no_mangle]
pub fn parse(source: &str) -> JsValue {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    JsValue::from_serde(&program.unwrap()).unwrap()
}
