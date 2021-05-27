use crate::Statement;
use crate::Expression;
use crate::ParserError;
use crate::{Function, FunctionParameter};

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

    fn match_token(lexer: &mut Lexer<'p>, token: Token<'p>) -> Result<Statement, ParserError<'p>> {
        let kind = token.kind;

        Ok(match kind {
            TokenType::OpenTag => {
                Statement::OpenTag
            },
            TokenType::Echo => {
                let expression = Parser::parse_expression(lexer)?;

                Parser::expect_token(lexer, TokenType::SemiColon, ";")?;

                Statement::Echo(expression)
            },
            TokenType::Function => {
                let identifier = Parser::expect_token(lexer, TokenType::Identifier, "")?;

                Parser::expect_token(lexer, TokenType::LeftParen, "(")?;

                let mut parameters: Vec<FunctionParameter> = Vec::new();

                loop {
                    let next = lexer.next();

                    if matches!(next, Some(Token { kind: TokenType::RightParen, .. })) {
                        break;
                    }

                    let mut name = String::new();
                    let mut type_hint = None;

                    match lexer.next() {
                        Some(t @ Token { kind: TokenType::Identifier, .. }) => {
                            type_hint = Some(t.slice.to_string())
                        },
                        Some(t @ Token { kind: TokenType::Variable, .. }) => {
                            let mut buffer: String = t.slice.to_string();
                            buffer.remove(0);

                            name = buffer;
                        },
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => return Err(ParserError::Unknown)
                    }
                }

                let mut return_type_hint = None;
                let next = lexer.next();

                if matches!(next, Some(Token { kind: TokenType::Colon, .. })) {
                    let return_type_token = Parser::expect_token(lexer, TokenType::Identifier, "")?;

                    return_type_hint = Some(return_type_token.slice.to_string());

                    Parser::expect_token(lexer, TokenType::LeftBrace, "{")?;
                } else if next.is_some() && ! matches!(next, Some(Token { kind: TokenType::LeftBrace, .. })) {
                    let next = next.unwrap();

                    return Err(ParserError::ExpectedToken {
                        expected_type: TokenType::LeftBrace,
                        expected_slice: "{",
                        got_type: next.kind,
                        got_slice: next.slice,
                    })
                }

                let body = Vec::new();

                Statement::Function(Function::new(
                    identifier.slice.to_owned(),
                    parameters,
                    body,
                    return_type_hint
                ))
            },
            TokenType::String => {
                let mut buffer: String = token.slice.to_string();

                buffer.remove(0);
                buffer.pop();

                lexer.next();

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

    fn expect_token(lexer: &mut Lexer<'p>, kind: TokenType, slice: &'p str) -> Result<Token<'p>, ParserError<'p>> {
        let next = lexer.next();

        if next.is_none() {
            Err(ParserError::UnexpectedEndOfFile)
        } else {
            let token = next.unwrap();

            if token.kind != kind {
                Err(ParserError::ExpectedToken {
                    expected_type: kind,
                    expected_slice: slice,
                    got_type: token.kind,
                    got_slice: token.slice,
                })
            } else {
                Ok(token)
            }
        }
    }

    fn parse_expression(lexer: &mut Lexer<'p>) -> Result<Expression, ParserError<'p>> {
        let next = lexer.next();

        if next.is_none() {
            return Err(ParserError::UnexpectedEndOfFile)
        }

        let next = next.unwrap();

        let mut lhs = match next.kind {
            TokenType::String => {
                Expression::String(next.slice.to_owned())
            },
            TokenType::Integer => {
                Expression::Integer(next.slice.parse::<i64>()?)
            },
            TokenType::Float => {
                Expression::Float(next.slice.parse::<f64>()?)
            }
            _ => unimplemented!()
        };

        Ok(lhs)
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