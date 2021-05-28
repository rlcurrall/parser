use serde::Serialize;
use tusk_lexer::TokenType;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl BinaryOp {
    pub fn from_token_type(kind: TokenType) -> Self {
        use TokenType::*;

        match kind {
            Plus => Self::Add,
            Minus => Self::Subtract,
            Asterisk => Self::Multiply,
            Slash => Self::Divide,
            Percent => Self::Modulo,
            _ => unreachable!(),
        }
    }
}
