use tusk_lexer::TokenType;

pub struct BindingPower;

impl BindingPower {
    pub fn postfix(kind: TokenType) -> Option<(u8, ())> {
        Some(match kind {
            TokenType::LeftBracket | TokenType::Arrow | TokenType::LeftParen => (19, ()),
            _ => return None,
        })
    }

    pub fn infix(kind: TokenType) -> Option<(u8, u8)> {
        use TokenType::*;

        Some(match kind {
            Asterisk | Slash => (98, 99),
            BitwiseLeftShift | BitwiseRightShift => (96, 97),
            BitwiseAnd => (13, 14),
            BitwiseOr => (11, 12),
            Plus | Minus | Period => (9, 10),
            GreaterThan | LessThan | GreaterThanEquals | LessThanEquals => (7, 8),
            Equals | DoubleArrow => (1, 2),
            _ => return None,
        })
    }
}
