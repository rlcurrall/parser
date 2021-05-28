use tusk_lexer::TokenType;

pub struct BindingPower;

impl BindingPower {

    pub fn postfix(kind: TokenType) -> Option<(u8, ())> {
        Some(match kind {
            _ => return None,
        })
    }

    pub fn infix(kind: TokenType) -> Option<(u8, u8)> {
        Some(match kind {
            TokenType::Period => (11, 12),
            TokenType::Equals => (2, 1),
            _ => return None
        })
    }
}