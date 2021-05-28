use crate::Class;
use crate::Else;
use crate::Expression;
use crate::Function;
use crate::If;
use crate::Property;

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Statement {
    OpenTag,
    Echo(Expression),
    Return(Expression),
    Expression(Expression),
    Function(Function),
    Class(Class),
    Property(Property),
    If(If),
    Else(Else),
}
