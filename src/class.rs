use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Class {
    name: String,
    implements: Vec<String>,
    extends: String,
}

impl Class {
    pub fn new(name: String, implements: Vec<String>, extends: String) -> Self {
        Self {
            name,
            implements,
            extends,
        }
    }
}
