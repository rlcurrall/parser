use crate::Class;
use crate::Expression;
use crate::Function;
use crate::Property;
use crate::If;
use crate::Else;

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
    Else(Else)
}
