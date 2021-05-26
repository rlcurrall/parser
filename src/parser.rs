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
            self.match_token(token)
        } else {
            Err(ParserError::Unknown)
        }
    }

    pub fn match_token(&mut self, token: Token) -> Result<Statement, ParserError> {
        Ok(match token.kind {
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
            _ => return Err(ParserError::Unknown)
        })
    }

    pub fn all(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut program = Vec::new();

        while let Some(token) = self.lexer.next() {
            let statement = self.match_token(token)?;

            program.push(statement);
        }

        Ok(program)
    }
}