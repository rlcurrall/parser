use crate::Expression;
use crate::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    OpenTag,
    Echo(Expression),
    Return(Expression),
    Expression(Expression),
    Function(Function)
}