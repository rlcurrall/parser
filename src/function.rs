use crate::Expression;
use crate::Statement;
use crate::{Flag, Flaggable};

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    parameters: Vec<FunctionParameter>,
    body: Vec<Statement>,
    return_type_hint: Option<String>,
    flags: Vec<Flag>
}

impl Function {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameter>,
        body: Vec<Statement>,
        return_type_hint: Option<String>,
        flags: Vec<Flag>
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            return_type_hint,
            flags
        }
    }
}

impl Flaggable for Function {

    fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag)
    }

    fn has_flag(&self, flag: Flag) -> bool {
        self.flags.contains(&flag)
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
