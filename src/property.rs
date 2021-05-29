use crate::Expression;
use crate::{Flag, Flaggable};
use crate::Nullable;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Property {
    pub name: String,
    flags: Vec<Flag>,
    type_hint: Option<String>,
    default: Option<Expression>,
}

impl Property {
    pub fn new(name: String, flags: Vec<Flag>, type_hint: Option<String>, default: Option<Expression>) -> Self {
        Self {
            name,
            flags,
            type_hint,
            default,
        }
    }
}

impl Nullable for Property {
    fn is_nullable(&self) -> bool {
        if self.type_hint.is_none() {
            return false;
        }

        self.type_hint.clone().unwrap().starts_with("?")
    }
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

    fn has_visiblity_flag(&self) -> bool {
        self.flags.clone().into_iter().filter(|flag| flag.is_visibility_flag()).count() >= 1
    }
}
