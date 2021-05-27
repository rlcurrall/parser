use crate::Class;
use crate::Expression;
use crate::ParserError;
use crate::Statement;
use crate::{Function, FunctionParameter};

use std::iter::Iterator;
use tusk_lexer::{Lexer, Token, TokenType};

type Program = Vec<Statement>;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
}

impl<'p> Iterator for Parser<'p> {
    type Item = Result<Statement, ParserError<'p>>;

    fn next(&mut self) -> Option<Result<Statement, ParserError<'p>>> {
        if let Some(token) = self.lexer.next() {
            Some(Parser::match_token(&mut self.lexer, token))
        } else {
            Some(Err(ParserError::Unknown))
        }
    }
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self { lexer }
    }

    fn match_token(lexer: &mut Lexer<'p>, token: Token<'p>) -> Result<Statement, ParserError<'p>> {
        let kind = token.kind;

        Ok(match kind {
            TokenType::OpenTag => Statement::OpenTag,
            TokenType::Echo => {
                let expression = Parser::parse_expression(lexer)?;

                Parser::expect_token(lexer, TokenType::SemiColon, ";")?;

                Statement::Echo(expression)
            }
            TokenType::Return => {
                let expression = Parser::parse_expression(lexer)?;

                Parser::expect_token(lexer, TokenType::SemiColon, ";")?;

                Statement::Return(expression)
            }
            TokenType::Class => {
                let name = Parser::expect_token(lexer, TokenType::Identifier, "")?;
                let mut implements = Vec::new();
                let mut extends = String::new();

                'outer: loop {
                    let next = lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::Extends,
                            ..
                        }) => {
                            if implements.len() >= 1 {
                                let t = next.unwrap();

                                return Err(ParserError::UnexpectedToken(t.kind, t.slice));
                            }

                            let identifier =
                                Parser::expect_token(lexer, TokenType::Identifier, "")?;

                            extends = identifier.slice.to_string();
                        }
                        Some(Token {
                            kind: TokenType::Implements,
                            ..
                        }) => {
                            let identifier =
                                Parser::expect_token(lexer, TokenType::Identifier, "")?;

                            implements.push(identifier.slice.to_string());

                            loop {
                                let next = lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::Identifier,
                                        ..
                                    }) => {
                                        implements.push(next.unwrap().slice.to_string());
                                    }
                                    Some(Token {
                                        kind: TokenType::Comma,
                                        ..
                                    }) => {
                                        let identifier =
                                            Parser::expect_token(lexer, TokenType::Identifier, "")?;

                                        implements.push(identifier.slice.to_string());
                                    }
                                    Some(Token {
                                        kind: TokenType::LeftBrace,
                                        ..
                                    }) => {
                                        if implements.len() >= 1 {
                                            break 'outer;
                                        }

                                        continue;
                                    }
                                    None => return Err(ParserError::UnexpectedEndOfFile),
                                    _ => {
                                        let t = next.unwrap();

                                        return Err(ParserError::UnexpectedToken(t.kind, t.slice));
                                    }
                                }
                            }
                        }
                        Some(Token {
                            kind: TokenType::LeftBrace,
                            ..
                        }) => break,
                        _ => return Err(ParserError::Unknown),
                    }
                }

                Parser::expect_token(lexer, TokenType::RightBrace, "}")?;

                Statement::Class(Class::new(name.slice.to_owned(), implements, extends))
            }
            TokenType::Function => {
                let identifier = Parser::expect_token(lexer, TokenType::Identifier, "")?;

                Parser::expect_token(lexer, TokenType::LeftParen, "(")?;

                let mut parameters: Vec<FunctionParameter> = Vec::new();

                loop {
                    let mut next = lexer.next();

                    match next {
                        // break when finding a ), no more parameters
                        Some(Token {
                            kind: TokenType::RightParen,
                            ..
                        }) => break,
                        // consume trailing commas..
                        Some(Token {
                            kind: TokenType::Comma,
                            ..
                        }) => {
                            next = lexer.next();
                        }
                        Some(Token {
                            kind: TokenType::Identifier | TokenType::Variable,
                            ..
                        }) => (),
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let t = next.unwrap();

                            return Err(ParserError::UnexpectedToken(t.kind, t.slice));
                        }
                    }

                    let mut name = String::new();
                    let mut type_hint = None;

                    match next {
                        Some(
                            t
                            @
                            Token {
                                kind: TokenType::Identifier,
                                ..
                            },
                        ) => type_hint = Some(t.slice.to_string()),
                        Some(
                            t
                            @
                            Token {
                                kind: TokenType::Variable,
                                ..
                            },
                        ) => {
                            let mut buffer: String = t.slice.to_string();
                            buffer.remove(0);

                            name = buffer;
                        }
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => return Err(ParserError::Unknown),
                    }

                    if type_hint.is_some() {
                        let variable = Parser::expect_token(lexer, TokenType::Variable, "")?;

                        let mut buffer: String = variable.slice.to_string();
                        buffer.remove(0);

                        name = buffer;
                    }

                    let next = lexer.next();
                    let mut default = None;

                    if matches!(
                        next,
                        Some(Token {
                            kind: TokenType::Equals,
                            ..
                        })
                    ) {
                        default = Some(Parser::parse_expression(lexer)?);
                    }

                    parameters.push(FunctionParameter::new(name, type_hint, default))
                }

                let mut return_type_hint = None;
                let next = lexer.next();

                if matches!(
                    next,
                    Some(Token {
                        kind: TokenType::Colon,
                        ..
                    })
                ) {
                    let return_type_token = Parser::expect_token(lexer, TokenType::Identifier, "")?;

                    return_type_hint = Some(return_type_token.slice.to_string());

                    Parser::expect_token(lexer, TokenType::LeftBrace, "{")?;
                } else if next.is_some()
                    && !matches!(
                        next,
                        Some(Token {
                            kind: TokenType::LeftBrace,
                            ..
                        })
                    )
                {
                    let next = next.unwrap();

                    return Err(ParserError::ExpectedToken {
                        expected_type: TokenType::LeftBrace,
                        expected_slice: "{",
                        got_type: next.kind,
                        got_slice: next.slice,
                    });
                }

                let mut body = Vec::new();

                loop {
                    let next = lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace,
                            ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = Parser::match_token(lexer, next.unwrap())?;

                            body.push(statement);
                        }
                    }
                }

                Statement::Function(Function::new(
                    identifier.slice.to_owned(),
                    parameters,
                    body,
                    return_type_hint,
                ))
            }
            TokenType::String => {
                let mut buffer: String = token.slice.to_string();

                buffer.remove(0);
                buffer.pop();

                lexer.next();

                Statement::Expression(Expression::String(buffer))
            }
            TokenType::Integer => {
                Statement::Expression(Expression::Integer(token.slice.parse::<i64>()?))
            }
            TokenType::Float => {
                Statement::Expression(Expression::Float(token.slice.parse::<f64>()?))
            }
            _ => return Err(ParserError::UnexpectedToken(kind, token.slice)),
        })
    }

    fn expect_token(
        lexer: &mut Lexer<'p>,
        kind: TokenType,
        slice: &'p str,
    ) -> Result<Token<'p>, ParserError<'p>> {
        let next = lexer.next();

        if let Some(token) = next {
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
        } else {
            Err(ParserError::UnexpectedEndOfFile)
        }
    }

    fn parse_expression(lexer: &mut Lexer<'p>) -> Result<Expression, ParserError<'p>> {
        let next = lexer.next();

        if next.is_none() {
            return Err(ParserError::UnexpectedEndOfFile);
        }

        let next = next.unwrap();

        let lhs = match next.kind {
            TokenType::String => {
                let mut buffer: String = next.slice.to_owned();
                // remove the quotes
                buffer.remove(0);
                buffer.pop();

                Expression::String(buffer)
            }
            TokenType::Integer => Expression::Integer(next.slice.parse::<i64>()?),
            TokenType::Float => Expression::Float(next.slice.parse::<f64>()?),
            _ => unimplemented!(),
        };

        Ok(lhs)
    }

    #[allow(clippy::while_let_on_iterator)]
    pub fn all(&mut self) -> Result<Program, ParserError> {
        let mut program = Vec::new();

        while let Some(token) = self.lexer.next() {
            let statement = Parser::match_token(&mut self.lexer, token)?;

            program.push(statement);
        }

        Ok(program)
    }
}
