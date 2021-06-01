use serde::Serialize;
use tusk_lexer::TokenType;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseXor,
    And,
    Or,
}

impl From<TokenType> for BinaryOp {
    fn from(kind: TokenType) -> Self {
        use TokenType::*;

        match kind {
            Plus => Self::Add,
            Minus => Self::Subtract,
            Asterisk => Self::Multiply,
            Slash => Self::Divide,
            Percent => Self::Modulo,
            BitwiseAnd => Self::BitwiseAnd,
            BitwiseOr => Self::BitwiseOr,
            BitwiseLeftShift => Self::BitwiseLeftShift,
            BitwiseRightShift => Self::BitwiseRightShift,
            BitwiseXor => Self::BitwiseXor,
            And => Self::And,
            Or => Self::Or,
            _ => unreachable!(),
        }
    }
}
