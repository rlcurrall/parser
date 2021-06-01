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
            BitwiseAnd => (94, 95),
            BitwiseXor => (92, 93),
            BitwiseOr => (90, 91),
            Plus | Minus | Period => (88, 89),
            And => (86, 87),
            Or => (84, 85),
            GreaterThan | LessThan | GreaterThanEquals | LessThanEquals => (7, 8),
            Equals | DoubleArrow => (1, 2),
            _ => return None,
        })
    }
}
