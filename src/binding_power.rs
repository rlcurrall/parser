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
            Asterisk | Slash => (13, 14),
            Plus | Minus | Period => (11, 12),
            GreaterThan | LessThan | GreaterThanEquals | LessThanEquals => (9, 10),
            Equals | DoubleArrow => (2, 1),
            _ => return None,
        })
    }
}
