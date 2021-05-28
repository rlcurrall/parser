use crate::Expression;
use crate::Statement;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct If {
    condition: Expression,
    then: Vec<Statement>,
    else_ifs: Vec<Statement>,
    r#else: Option<Box<Statement>>,
}

impl If {
    pub fn new(
        condition: Expression,
        then: Vec<Statement>,
        else_ifs: Vec<Statement>,
        r#else: Option<Box<Statement>>,
    ) -> Self {
        Self {
            condition,
            then,
            else_ifs,
            r#else,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Else {
    then: Vec<Statement>,
}

impl Else {
    pub fn new(then: Vec<Statement>) -> Self {
        Self { then }
    }
}
