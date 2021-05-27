use tusk_lexer::Lexer;
use wasm_bindgen::prelude::*;

mod class;
mod error;
mod expression;
mod function;
mod parser;
mod statement;
mod flag;

pub use class::Class;
pub use error::ParserError;
pub use expression::Expression;
pub use function::Function;
pub use function::FunctionParameter;
pub use parser::Parser;
pub use statement::Statement;
pub use flag::Flag;
pub use flag::Flaggable;

#[wasm_bindgen]
#[no_mangle]
pub fn parse(source: &str) -> JsValue {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    JsValue::from_serde(&program.unwrap()).unwrap()
}
