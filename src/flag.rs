use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Flag {
    Final,
    Public,
    Protected,
    Private,
    Static,
    Abstract,
}

pub trait Flaggable {
    fn add_flag(&mut self, flag: Flag);
    fn has_flag(&self, flag: Flag) -> bool;
    fn has_flags(&self) -> bool;
}
