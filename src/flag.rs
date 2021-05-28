use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Final,
    Public,
    Protected,
    Private,
    Static,
    Abstract,
}

impl Flag {
    pub fn is_visibility_flag(&self) -> bool {
        match self {
            Flag::Public | Flag::Protected | Flag::Private => true,
            _ => false,
        }
    }
}

pub trait Flaggable {
    fn add_flag(&mut self, flag: Flag);
    fn has_flag(&self, flag: Flag) -> bool;
    fn has_flags(&self) -> bool;
    fn has_visiblity_flag(&self) -> bool;
}
