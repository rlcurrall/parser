use crate::BinaryOp;
use crate::Function;

use serde::Serialize;
use tusk_lexer::TokenType;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expression {
    True,
    False,
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    TypedVariable(String, String),
    Identifier(String),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    Concat(Box<Expression>, Box<Expression>),
    Array(Vec<Expression>),
    ArrayAccess(Box<Expression>, Option<Box<Expression>>),
    ArrayItem { key: Box<Expression>, value: Box<Expression> },
    PropertyAccess(Box<Expression>, Box<Expression>),
    New {
        class: Box<Expression>,
        args: Vec<Expression>,
    },
    Call {
        target: Box<Expression>,
        args: Vec<Expression>,
    },
    MethodCall {
        target: Box<Expression>,
        method: Box<Expression>,
        args: Vec<Expression>
    },
    Closure(Function)
}

impl Expression {
    pub fn make_infix(lhs: Expression, operator: &TokenType, rhs: Expression) -> Self {
        use TokenType::*;

        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);

        match *operator {
            Plus | Minus | Asterisk | Slash | Percent | BitwiseAnd | BitwiseOr | BitwiseLeftShift | BitwiseRightShift | BitwiseXor | And | Or => Self::Binary(lhs, BinaryOp::from(*operator), rhs),
            Period => Self::Concat(lhs, rhs),
            DoubleArrow => Self::ArrayItem { key: lhs, value: rhs },
            Equals => Self::Assign(lhs, rhs),
            _ => unimplemented!(),
        }
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}
