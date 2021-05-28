#![feature(bindings_after_at)]

use tusk_lexer::Lexer;
use wasm_bindgen::prelude::*;

mod binary_op;
mod binding_power;
mod class;
mod error;
mod expression;
mod flag;
mod function;
mod if_statement;
mod parser;
mod property;
mod statement;

pub use binary_op::BinaryOp;
pub use binding_power::BindingPower;
pub use class::Class;
pub use error::ParserError;
pub use expression::Expression;
pub use flag::Flag;
pub use flag::Flaggable;
pub use function::Function;
pub use function::FunctionParameter;
pub use if_statement::Else;
pub use if_statement::If;
pub use parser::Parser;
pub use property::Property;
pub use statement::Statement;

#[wasm_bindgen]
#[no_mangle]
pub fn parse(source: &str) -> JsValue {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.all();

    JsValue::from_serde(&program.unwrap()).unwrap()
}
