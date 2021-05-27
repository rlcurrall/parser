use crate::Expression;
use crate::Statement;

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    parameters: Vec<FunctionParameter>,
    body: Vec<Statement>,
    return_type_hint: Option<String>,
}

impl Function {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameter>,
        body: Vec<Statement>,
        return_type_hint: Option<String>,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            return_type_hint,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    name: String,
    type_hint: Option<String>,
    default: Option<Expression>,
}

impl FunctionParameter {
    pub fn new(name: String, type_hint: Option<String>, default: Option<Expression>) -> Self {
        Self {
            name,
            type_hint,
            default,
        }
    }
}
