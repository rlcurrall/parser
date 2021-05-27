use crate::Expression;
use crate::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    User {
        name: String,
        parameters: Vec<FunctionParameter>,
        body: Vec<Statement>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    name: String,
    type_hint: String,
    default: Option<Expression>,
}