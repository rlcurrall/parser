use crate::Statement;
use crate::Expression;
use crate::ParserError;

use tusk_lexer::{Lexer, Token, TokenType};

pub struct Parser<'p> {
    lexer: Lexer<'p>
}

impl<'p> Parser<'p> {

    pub fn new(lexer: Lexer<'p>) -> Self {
        Self { lexer }
    }

    pub fn next(&mut self) -> Result<Statement, ParserError> {
        if let Some(token) = self.lexer.next() {
            Parser::match_token(&mut self.lexer, token)
        } else {
            Err(ParserError::Unknown)
        }
    }

    pub fn match_token(lexer: &mut Lexer, token: Token<'p>) -> Result<Statement, ParserError<'p>> {
        let kind = token.kind;

        Ok(match kind {
            TokenType::OpenTag => {
                Statement::OpenTag
            },
            TokenType::Echo => {
                Statement::Echo
            },
            TokenType::String => {
                let mut buffer: String = token.slice.to_string();

                buffer.remove(0);
                buffer.pop();

                Statement::Expression(Expression::String(buffer))
            },
            TokenType::Integer => {
                Statement::Expression(Expression::Integer(token.slice.parse::<i64>()?))
            },
            TokenType::Float => {
                Statement::Expression(Expression::Float(token.slice.parse::<f64>()?))
            }
            _ => {
                return Err(ParserError::UnexpectedToken(kind, token.slice))
            }
        })
    }

    pub fn all(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut program = Vec::new();

        while let Some(token) = self.lexer.next() {
            let statement = Parser::match_token(&mut self.lexer, token)?;

            program.push(statement);
        }

        Ok(program)
    }
}