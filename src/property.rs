use crate::Expression;
use crate::{Flag, Flaggable};
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Property {
    name: String,
    flags: Vec<Flag>,
    type_hint: String,
    default: Expression,
}

impl Flaggable for Property {
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
