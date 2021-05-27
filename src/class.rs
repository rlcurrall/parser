use crate::Statement;

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Class {
    name: String,
    implements: Vec<String>,
    extends: String,
    body: Vec<Statement>
}

impl Class {
    pub fn new(name: String, implements: Vec<String>, extends: String, body: Vec<Statement>) -> Self {
        Self {
            name,
            implements,
            extends,
            body
        }
    }
}
