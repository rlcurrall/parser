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
    Break,
    Continue(Option<Expression>),
    DocBlock(String),
    Echo(Expression),
    Return(Expression),
    Expression(Expression),
    Function(Function),
    Class(Class),
    Property(Property),
    If(If),
    Else(Else),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    DoWhile {
        condition: Expression,
        body: Vec<Statement>
    },
    Foreach {
        expression: Expression,
        key_var: Option<Expression>,
        value_var: Expression,
        body: Vec<Statement>,
    },
    Use(Expression),
    UseTrait(Expression),
}
