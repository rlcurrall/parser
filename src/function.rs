use crate::Expression;
use crate::Statement;
use crate::{Flag, Flaggable};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ClosureType {
    Long,
    Short,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub parameters: Vec<FunctionParameter>,
    pub body: Vec<Statement>,
    pub return_type_hint: Option<String>,
    pub flags: Vec<Flag>,
    pub closure_type: Option<ClosureType>,
}

impl Function {
    pub fn new(name: Option<String>, parameters: Vec<FunctionParameter>, body: Vec<Statement>, return_type_hint: Option<String>, flags: Vec<Flag>, closure_type: Option<ClosureType>) -> Self {
        Self {
            name,
            parameters,
            body,
            return_type_hint,
            flags,
            closure_type,
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

    fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }

    fn has_visiblity_flag(&self) -> bool {
        self.flags.clone().into_iter().filter(|flag| flag.is_visibility_flag()).count() > 1
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub type_hint: Option<String>,
    pub default: Option<Expression>,
}

impl FunctionParameter {
    pub fn new(name: String, type_hint: Option<String>, default: Option<Expression>) -> Self {
        Self { name, type_hint, default }
    }
}
