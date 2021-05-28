use crate::Statement;
use crate::{Flag, Flaggable};

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Class {
    name: String,
    implements: Vec<String>,
    extends: String,
    body: Vec<Statement>,
    flags: Vec<Flag>,
}

impl Class {
    pub fn new(
        name: String,
        implements: Vec<String>,
        extends: String,
        body: Vec<Statement>,
        flags: Vec<Flag>,
    ) -> Self {
        Self {
            name,
            implements,
            extends,
            body,
            flags,
        }
    }
}

impl Flaggable for Class {
    fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag)
    }

    fn has_flag(&self, flag: Flag) -> bool {
        self.flags.contains(&flag)
    }

    fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }
}
