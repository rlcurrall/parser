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