use crate::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression)
}