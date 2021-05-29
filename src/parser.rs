use crate::BindingPower;
use crate::Class;
use crate::Expression;
use crate::ParserError;
use crate::Statement;
use crate::{Else, If};
use crate::{Flag, Flaggable};
use crate::{Function, FunctionParameter};
use crate::Property;

use std::iter::Iterator;
use std::borrow::BorrowMut;
use tusk_lexer::{Lexer, Token, TokenType};

type Program = Vec<Statement>;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self { lexer }
    }

    #[allow(clippy::needless_collect)]
    fn match_token(&mut self, token: Token<'p>) -> Result<Statement, ParserError<'p>> {
        let kind = token.kind;

        Ok(match kind {
            TokenType::OpenTag => Statement::OpenTag,
            TokenType::Echo => {
                let expression = self.parse_expression(0, None)?;

                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::Echo(expression)
            }
            TokenType::If => {
                self.expect_token(TokenType::LeftParen, "(")?;

                let condition = self.parse_expression(0, None)?;

                self.expect_token(TokenType::RightParen, ")")?;
                self.expect_token(TokenType::LeftBrace, "{")?;

                let mut body = Vec::new();
                let mut did_find_right_brace = false;

                loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace,
                            ..
                        }) => {
                            did_find_right_brace = true;

                            break;
                        }
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = self.match_token(next.unwrap())?;

                            body.push(statement);
                        }
                    }
                }

                if !did_find_right_brace {
                    self.expect_token(TokenType::RightBrace, "}")?;
                }

                let mut else_ifs = Vec::new();
                let mut r#else = None;

                loop {
                    let next = self.lexer.next();

                    let next = match next {
                        Some(_) => next.unwrap(),
                        None => break,
                    };

                    match next.kind {
                        TokenType::Else => {
                            self.expect_token(TokenType::LeftBrace, "{")?;

                            let mut body = Vec::new();
                            let mut did_find_right_brace = false;

                            loop {
                                let next = self.lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::RightBrace,
                                        ..
                                    }) => {
                                        did_find_right_brace = true;

                                        break;
                                    }
                                    None => return Err(ParserError::UnexpectedEndOfFile),
                                    _ => {
                                        let statement = self.match_token(next.unwrap())?;

                                        body.push(statement);
                                    }
                                }
                            }

                            if !did_find_right_brace {
                                self.expect_token(TokenType::RightBrace, "}")?;
                            }

                            r#else = Some(Box::new(Statement::Else(Else::new(body))))
                        }
                        _ => return Err(ParserError::UnexpectedToken(next.kind, next.slice)),
                    }
                }

                Statement::If(If::new(condition, body, else_ifs, r#else))
            }
            TokenType::Return => {
                let expression = self.parse_expression(0, None)?;

                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::Return(expression)
            }
            flag
            @
            (TokenType::Public
            | TokenType::Protected
            | TokenType::Private
            | TokenType::Final
            | TokenType::Abstract
            | TokenType::Static) => {
                let next = self.lexer.next();

                if next.is_none() {
                    return Err(ParserError::UnexpectedEndOfFile);
                }

                let mut statement = self.match_token(next.unwrap())?;

                let flag_type = match flag {
                    TokenType::Public => Flag::Public,
                    TokenType::Protected => Flag::Protected,
                    TokenType::Private => Flag::Private,
                    TokenType::Final => Flag::Final,
                    TokenType::Abstract => Flag::Abstract,
                    TokenType::Static => Flag::Static,
                    _ => unreachable!(),
                };

                match statement {
                    Statement::Function(ref mut function) => {
                        if flag_type == Flag::Final && function.has_flag(Flag::Abstract) {
                            return Err(ParserError::FlagNotAllowed(flag_type, "abstract methods.".to_owned()));
                        }

                        if flag_type == Flag::Abstract && function.has_flag(Flag::Final) {
                            return Err(ParserError::FlagNotAllowed(flag_type, "final methods".to_owned()));
                        }

                        function.add_flag(flag_type)
                    }
                    Statement::Class(ref mut class) => {
                        if matches!(flag_type, Flag::Final) && class.has_flag(Flag::Abstract) {
                            return Err(ParserError::FlagNotAllowed(flag_type, "abstract classes.".to_owned()));
                        }

                        if matches!(flag_type, Flag::Abstract) && class.has_flag(Flag::Final) {
                            return Err(ParserError::FlagNotAllowed(flag_type, "final classes.".to_owned()));
                        }

                        class.add_flag(flag_type)
                    }
                    Statement::Expression(Expression::TypedVariable(ref type_hint, ref variable)) => {
                        let mut property = Property::new(variable.clone(), Vec::new(), Some(type_hint.clone()), None);

                        property.add_flag(flag_type);

                        statement = Statement::Property(property)
                    },
                    Statement::Property(ref mut property) => {
                        if flag_type == Flag::Final || flag_type == Flag::Abstract {
                            return Err(ParserError::FlagNotAllowed(flag_type, "properties".to_owned()));
                        }

                        if property.has_flag(flag_type) {
                            return Err(ParserError::DuplicateFlag(flag_type));
                        }

                        if flag_type.is_visibility_flag() && property.has_visiblity_flag() {
                            return Err(ParserError::FlagNotAllowed(flag_type, "properties with existing visiblity flags".to_owned()))
                        }

                        property.add_flag(flag_type)
                    },
                    Statement::Expression(Expression::Assign(ref variable, ref default)) => {

                    },
                    _ => {
                        return Err(ParserError::Unknown)
                    },
                }

                statement
            }
            TokenType::Class => {
                let name = self.expect_token(TokenType::Identifier, "")?;
                let mut implements = Vec::new();
                let mut extends = String::new();

                'outer: loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::Extends,
                            ..
                        }) => {
                            if !implements.is_empty() {
                                let t = next.unwrap();

                                return Err(ParserError::UnexpectedToken(t.kind, t.slice));
                            }

                            let identifier = self.expect_token(TokenType::Identifier, "")?;

                            extends = identifier.slice.to_string();
                        }
                        Some(Token {
                            kind: TokenType::Implements,
                            ..
                        }) => {
                            let identifier = self.expect_token(TokenType::Identifier, "")?;

                            implements.push(identifier.slice.to_string());

                            loop {
                                let next = self.lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::Identifier,
                                        ..
                                    }) => {
                                        implements.push(next.unwrap().slice.to_string());
                                    }
                                    Some(Token {
                                        kind: TokenType::Comma, ..
                                    }) => {
                                        let identifier = self.expect_token(TokenType::Identifier, "")?;

                                        implements.push(identifier.slice.to_string());
                                    }
                                    Some(Token {
                                        kind: TokenType::LeftBrace,
                                        ..
                                    }) => {
                                        if !implements.is_empty() {
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

                let mut body: Vec<Statement> = Vec::new();

                loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace,
                            ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = self.match_token(next.unwrap())?;

                            match &statement {
                                Statement::Function(Function {
                                    name: function_name, ..
                                }) => {
                                    let matches: Vec<Statement> = body
                                        .clone()
                                        .into_iter()
                                        .filter(|statement| match statement {
                                            Statement::Function(function) => function.name == *function_name,
                                            _ => false,
                                        })
                                        .collect();

                                    if !matches.is_empty() {
                                        return Err(ParserError::MethodAlreadyExists(function_name.clone()));
                                    }
                                }
                                Statement::Property(Property {
                                    name: property_name, ..
                                }) => {
                                    let matches: Vec<Statement> = body
                                        .clone()
                                        .into_iter()
                                        .filter(|statement| match statement {
                                            Statement::Property(property) => property.name == *property_name,
                                            _ => false,
                                        })
                                        .collect();

                                    if !matches.is_empty() {
                                        return Err(ParserError::PropertyAlreadyExists(property_name.clone()));
                                    }
                                }
                                _ => return Err(ParserError::UnexpectedStatement(statement)),
                            };

                            body.push(statement);
                        }
                    }
                }

                Statement::Class(Class::new(name.slice.to_owned(), implements, extends, body, Vec::new()))
            }
            TokenType::Function => {
                let identifier = self.expect_token(TokenType::Identifier, "")?;

                self.expect_token(TokenType::LeftParen, "(")?;

                let mut parameters: Vec<FunctionParameter> = Vec::new();

                loop {
                    let mut next = self.lexer.next();

                    match next {
                        // break when finding a ), no more parameters
                        Some(Token {
                            kind: TokenType::RightParen,
                            ..
                        }) => break,
                        // consume trailing commas..
                        Some(Token {
                            kind: TokenType::Comma, ..
                        }) => {
                            next = self.lexer.next();
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
                        let variable = self.expect_token(TokenType::Variable, "")?;

                        let mut buffer: String = variable.slice.to_string();
                        buffer.remove(0);

                        name = buffer;
                    }

                    let next = self.lexer.next();
                    let mut default = None;

                    if matches!(
                        next,
                        Some(Token {
                            kind: TokenType::Equals,
                            ..
                        })
                    ) {
                        default = Some(self.parse_expression(0, None)?);
                    }

                    parameters.push(FunctionParameter::new(name, type_hint, default))
                }

                let mut return_type_hint = None;
                let next = self.lexer.next();

                if matches!(
                    next,
                    Some(Token {
                        kind: TokenType::Colon,
                        ..
                    })
                ) {
                    let return_type_token = self.expect_token(TokenType::Identifier, "")?;

                    return_type_hint = Some(return_type_token.slice.to_string());

                    self.expect_token(TokenType::LeftBrace, "{")?;
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
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace,
                            ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = self.match_token(next.unwrap())?;
                            
                            body.push(statement);
                        }
                    }
                }

                Statement::Function(Function::new(
                    identifier.slice.to_owned(),
                    parameters,
                    body,
                    return_type_hint,
                    Vec::new(),
                ))
            }
            TokenType::String => {
                let mut buffer: String = token.slice.to_string();

                buffer.remove(0);
                buffer.pop();

                self.lexer.next();

                Statement::Expression(Expression::String(buffer))
            }
            TokenType::Integer => Statement::Expression(Expression::Integer(token.slice.parse::<i64>()?)),
            TokenType::Float => Statement::Expression(Expression::Float(token.slice.parse::<f64>()?)),
            _ => {
                let expression = self.parse_expression(0, Some(token))?;

                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::Expression(expression)
            }
        })
    }

    fn expect_token(&mut self, kind: TokenType, slice: &'p str) -> Result<Token<'p>, ParserError<'p>> {
        let next = self.lexer.next();

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

    fn parse_expression<'n>(
        &mut self,
        bp: u8,
        maybe_token: Option<Token>,
    ) -> Result<Expression, ParserError<'p>> {
        let next = if maybe_token.is_none() {
            self.lexer.next()
        } else {
            maybe_token
        };

        if next.is_none() {
            return Err(ParserError::UnexpectedEndOfFile);
        }

        let next = next.unwrap();

        let mut lhs = match next.kind {
            TokenType::String => {
                let mut buffer: String = next.slice.to_owned();
                // remove the quotes
                buffer.remove(0);
                buffer.pop();

                Expression::String(buffer)
            }
            TokenType::Integer => Expression::Integer(next.slice.parse::<i64>()?),
            TokenType::Float => Expression::Float(next.slice.parse::<f64>()?),
            TokenType::Variable => {
                let mut buffer = next.slice.to_string();
                // remove the $
                buffer.remove(0);

                Expression::Variable(buffer)
            }
            TokenType::True => Expression::from(true),
            TokenType::False => Expression::from(false),
            TokenType::Null => Expression::Null,
            TokenType::LeftBracket => {
                let mut items = Vec::new();
                let mut counter = 0;

                loop {
                    let next = self.lexer.peek();

                    match next {
                        Some(Token { kind: TokenType::RightBracket, .. }) => {
                            self.lexer.next();
                            
                            break
                        },
                        Some(Token { kind: TokenType::Comma, .. }) => {
                            self.lexer.next();

                            continue;
                        },
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let expression = self.parse_expression(0, None)?;

                            match expression {
                                Expression::ArrayItem { ref key, .. } => {
                                    match **key {
                                        Expression::Integer(i) => {
                                            counter = i + 1
                                        },
                                        Expression::Float(f) => {
                                            counter = (f as i64) + 1
                                        }
                                        _ => ()
                                    }

                                    items.push(expression)
                                },
                                _ => {
                                    let key = Expression::Integer(counter.clone());
                                    
                                    items.push(Expression::ArrayItem {
                                        key: Box::new(key), value: Box::new(expression),
                                    });

                                    counter += 1
                                },
                            }
                        }
                    }
                }
                
                Expression::Array(items)
            },
            TokenType::Identifier => {
                match self.lexer.clone().next() {
                    Some(Token { kind: TokenType::Variable, slice, .. }) => {
                        let mut buffer = slice.to_string();
                        // remove the $
                        buffer.remove(0);

                        self.lexer.next();

                        Expression::TypedVariable(next.slice.to_owned(), buffer) 
                    },
                    _ => Expression::Identifier(next.slice.to_owned())
                }
            },
            _ => {
                unimplemented!()
            }
        };

        loop {
            let next = self.lexer.peek();

            if next.is_none() {
                return Err(ParserError::UnexpectedEndOfFile);
            }

            let op = next.unwrap();

            if let Some((lbp, _)) = BindingPower::postfix(op.kind) {
                if lbp < bp {
                    break;
                }

                let op = self.lexer.next().unwrap();

                lhs = match op.kind {
                    TokenType::LeftBracket => {
                        let next = self.lexer.next();

                        let expression = match next {
                            Some(Token { kind: TokenType::RightBracket, .. }) => {  
                                None
                            },
                            None => return Err(ParserError::UnexpectedEndOfFile),
                            _ => {
                                let index = self.parse_expression(0, next)?;

                                self.expect_token(TokenType::RightBracket, "]")?;

                                Some(Box::new(index))
                            }
                        };

                        Expression::ArrayAccess(Box::new(lhs.clone()), expression)
                    },
                    _ => unreachable!()
                };
            } else if let Some((lbp, rbp)) = BindingPower::infix(op.kind) {
                if lbp < bp {
                    break;
                }

                let op = self.lexer.next().unwrap();

                let rhs = self.parse_expression(rbp, None)?;

                lhs = Expression::make_infix(lhs, &op.kind, rhs);

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    #[allow(clippy::while_let_on_iterator)]
    pub fn all(&'p mut self) -> Result<Program, ParserError> {
        let mut program = Vec::new();

        while let Some(token) = self.lexer.next() {
            let statement = self.match_token(token)?;

            program.push(statement);
        }

        Ok(program)
    }
}
