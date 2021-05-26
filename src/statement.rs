use crate::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    OpenTag,
    Echo(Expression),
    Expression(Expression)
}