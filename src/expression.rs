use serde::Serialize;
use tusk_lexer::TokenType;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    Assign(Box<Expression>, Box<Expression>),
    Concat(Box<Expression>, Box<Expression>),
}

impl Expression {

    pub fn make_infix(lhs: Expression, operator: &TokenType, rhs: Expression) -> Self {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);

        match *operator {
            TokenType::Period => Self::Concat(lhs, rhs),
            TokenType::Equals => Self::Assign(lhs, rhs),
            _ => unimplemented!()
        }
    }
}
