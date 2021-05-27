use crate::Expression;
use crate::Function;

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Statement {
    OpenTag,
    Echo(Expression),
    Return(Expression),
    Expression(Expression),
    Function(Function),
}
