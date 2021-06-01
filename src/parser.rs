use crate::BindingPower;
use crate::Class;
use crate::Expression;
use crate::ParserError;
use crate::Property;
use crate::Statement;
use crate::{Else, If};
use crate::{Flag, Flaggable};
use crate::Nullable;
use crate::{Function, FunctionParameter, ClosureType};

use std::borrow::BorrowMut;
use std::iter::Iterator;
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
            TokenType::DocBlockComment => Statement::DocBlock(token.slice.to_owned()),
            TokenType::Break => {
                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::Break
            },
            TokenType::Continue => {
                let next = self.lexer.peek();

                match next {
                    Some(Token { kind: TokenType::SemiColon, .. }) => {
                        self.lexer.next();

                        Statement::Continue(None)
                    },
                    None => return Err(ParserError::UnexpectedEndOfFile),
                    _ => {
                        let expression = self.parse_expression(0, None)?;  
                        
                        self.expect_token(TokenType::SemiColon, ";")?;

                        Statement::Continue(Some(expression))
                    }
                }
            },
            TokenType::Use => {
                let expression = self.parse_expression(0, None)?;

                match expression {
                    Expression::Identifier(..) => {
                        self.expect_token(TokenType::SemiColon, ";")?;
                        
                        Statement::Use(expression)
                    },
                    _ => return Err(ParserError::UnexpectedExpression(expression))
                }
            },
            TokenType::Echo => {
                let expression = self.parse_expression(0, None)?;

                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::Echo(expression)
            },
            TokenType::While => {
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
                            kind: TokenType::RightBrace, ..
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

                Statement::While { condition, body }
            },
            TokenType::Do => {
                self.expect_token(TokenType::LeftBrace, "{")?;

                let mut body = Vec::new();
                let mut did_find_right_brace = false;

                loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace, ..
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

                self.expect_token(TokenType::While, "while")?;
                self.expect_token(TokenType::LeftParen, "(")?;

                let condition = self.parse_expression(0, None)?;

                self.expect_token(TokenType::RightParen, ")")?;
                self.expect_token(TokenType::SemiColon, ";")?;

                Statement::DoWhile { condition, body }
            },
            TokenType::Foreach => {
                self.expect_token(TokenType::LeftParen, "(")?;

                let left_hand = self.parse_expression(0, None)?;

                self.expect_token(TokenType::As, "as")?;

                let right_hand = self.parse_expression(0, None)?;
                let mut key_var = None;
                let mut value_var: Expression;

                match right_hand {
                    Expression::Variable(..) => {
                        value_var = right_hand
                    },
                    Expression::ArrayItem { key, value } => {
                        key_var = Some(*key);
                        value_var = *value
                    },
                    _ => {
                        return Err(ParserError::UnexpectedExpression(right_hand))
                    }
                };

                self.expect_token(TokenType::RightParen, ")")?;
                self.expect_token(TokenType::LeftBrace, "{")?;

                let mut body = Vec::new();
                let mut did_find_right_brace = false;

                loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace, ..
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

                Statement::Foreach {
                    expression: left_hand,
                    key_var: key_var,
                    value_var: value_var,
                    body: body,
                }
            },
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
                            kind: TokenType::RightBrace, ..
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
                        TokenType::ElseIf => {
                            self.expect_left_paren()?;

                            let condition = self.parse_expression(0, None)?;

                            self.expect_right_paren()?;
                            self.expect_left_brace()?;

                            let mut body = Vec::new();
                            let mut did_find_right_brace = false;

                            loop {
                                let next = self.lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::RightBrace, ..
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

                            else_ifs.push(Statement::ElseIf(If::new(condition, body, Vec::new(), None)))
                        },
                        TokenType::Else => {
                            let mut condition = None;
                            let mut else_if = false;

                            match self.lexer.peek() {
                                Some(Token { kind: TokenType::If, .. }) => {
                                    self.lexer.next();
                                    else_if = true;
                                    self.expect_left_paren()?;
                                    condition = Some(self.parse_expression(0, None)?);
                                    self.expect_right_paren()?;
                                },
                                _ => (),
                                None => return Err(ParserError::UnexpectedEndOfFile),
                            };

                            self.expect_token(TokenType::LeftBrace, "{")?;

                            let mut body = Vec::new();
                            let mut did_find_right_brace = false;

                            loop {
                                let next = self.lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::RightBrace, ..
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

                            if else_if {
                                else_ifs.push(Statement::ElseIf(If::new(condition.unwrap(), body, Vec::new(), None)));
                            } else {
                                r#else = Some(Box::new(Statement::Else(Else::new(body))))
                            }
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
            flag @ (TokenType::Public | TokenType::Protected | TokenType::Private | TokenType::Final | TokenType::Abstract | TokenType::Static) => {
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
                    }
                    Statement::Property(ref mut property) => {
                        if flag_type == Flag::Final || flag_type == Flag::Abstract {
                            return Err(ParserError::FlagNotAllowed(flag_type, "properties".to_owned()));
                        }

                        if property.has_flag(flag_type) {
                            return Err(ParserError::DuplicateFlag(flag_type));
                        }

                        if flag_type.is_visibility_flag() && property.has_visiblity_flag() {
                            return Err(ParserError::FlagNotAllowed(flag_type, "properties with existing visiblity flags".to_owned()));
                        }

                        property.add_flag(flag_type)
                    }
                    Statement::Expression(Expression::Assign(ref variable, ref default)) => {}
                    _ => return Err(ParserError::Unknown),
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
                        Some(Token { kind: TokenType::Extends, .. }) => {
                            if !implements.is_empty() {
                                let t = next.unwrap();

                                return Err(ParserError::UnexpectedToken(t.kind, t.slice));
                            }

                            let identifier = self.expect_token(TokenType::Identifier, "")?;

                            extends = identifier.slice.to_string();
                        }
                        Some(Token {
                            kind: TokenType::Implements, ..
                        }) => {
                            let identifier = self.expect_token(TokenType::Identifier, "")?;

                            implements.push(identifier.slice.to_string());

                            loop {
                                let next = self.lexer.next();

                                match next {
                                    Some(Token {
                                        kind: TokenType::Identifier, ..
                                    }) => {
                                        implements.push(next.unwrap().slice.to_string());
                                    }
                                    Some(Token { kind: TokenType::Comma, .. }) => {
                                        let identifier = self.expect_token(TokenType::Identifier, "")?;

                                        implements.push(identifier.slice.to_string());
                                    }
                                    Some(Token {
                                        kind: TokenType::LeftBrace, ..
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
                            kind: TokenType::LeftBrace, ..
                        }) => break,
                        _ => return Err(ParserError::Unknown),
                    }
                }

                let mut body: Vec<Statement> = Vec::new();

                loop {
                    let next = self.lexer.next();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBrace, ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let mut statement = self.match_token(next.unwrap())?;

                            match &statement {
                                Statement::Function(Function { name: function_name, .. }) => {
                                    let matches: Vec<Statement> = body
                                        .clone()
                                        .into_iter()
                                        .filter(|statement| match statement {
                                            Statement::Function(function) => function.name == *function_name,
                                            _ => false,
                                        })
                                        .collect();

                                    if !matches.is_empty() {
                                        return Err(ParserError::MethodAlreadyExists(function_name.clone().unwrap()));
                                    }
                                }
                                Statement::Use(expression) => {
                                    statement = Statement::UseTrait(expression.clone())
                                },
                                Statement::Property(Property { name: property_name, .. }) => {
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
                            kind: TokenType::RightParen, ..
                        }) => break,
                        // consume trailing commas..
                        Some(Token { kind: TokenType::Comma, .. }) => {
                            next = self.lexer.next();
                        }
                        Some(Token {
                            kind: TokenType::Identifier | TokenType::NullableIdentifier | TokenType::Variable,
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
                            t @ Token {
                                kind: TokenType::Identifier | TokenType::NullableIdentifier, ..
                            }
                        ) => type_hint = Some(t.slice.to_string()),
                        Some(t @ Token { kind: TokenType::Variable, .. }) => {
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

                    if matches!(next, Some(Token { kind: TokenType::Equals, .. })) {
                        default = Some(self.parse_expression(0, None)?);
                    }

                    parameters.push(FunctionParameter::new(name, type_hint, default))
                }

                let mut return_type_hint = None;
                let next = self.lexer.next();

                if matches!(next, Some(Token { kind: TokenType::Colon, .. })) {
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
                            kind: TokenType::RightBrace, ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = self.match_token(next.unwrap())?;

                            body.push(statement);
                        }
                    }
                }

                Statement::Function(Function::new(Some(identifier.slice.to_owned()), parameters, body, return_type_hint, Vec::new(), None))
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

    fn parse_expression<'n>(&mut self, bp: u8, maybe_token: Option<Token>) -> Result<Expression, ParserError<'p>> {
        let next = if maybe_token.is_none() { self.lexer.next() } else { maybe_token };

        if next.is_none() {
            return Err(ParserError::UnexpectedEndOfFile);
        }

        let next = next.unwrap();

        let mut lhs = match next.kind {
            TokenType::New => {
                let mut class = self.parse_expression(0, None)?;
                let mut args = Vec::new();

                match class {
                    Expression::Identifier(..) => (),
                    Expression::Call { target, args: call_args } => {
                        class = *target.clone();
                        
                        for arg in call_args {
                            args.push(arg);
                        }
                    },
                    _ => return Err(ParserError::UnexpectedExpression(class))
                };

                Expression::New {
                    class: Box::new(class),
                    args: args,
                }
            },
            TokenType::Static => {
                let mut expression = self.parse_expression(0, None)?;

                match expression {
                    Expression::Closure(ref mut function) => {
                        if function.has_flags() {
                            return Err(ParserError::CanOnlyHaveFlag(Flag::Static, "Anonymous functions".to_owned()))
                        }

                        function.add_flag(Flag::Static);
                    },
                    _ => {
                        return Err(ParserError::UnexpectedExpression(expression))
                    }
                }

                expression
            },
            TokenType::ShortFunction => {
                self.expect_token(TokenType::LeftParen, "(")?;

                let mut parameters: Vec<FunctionParameter> = Vec::new();

                loop {
                    let mut next = self.lexer.next();

                    println!("{:?}", next);

                    match next {
                        // break when finding a ), no more parameters
                        Some(Token {
                            kind: TokenType::RightParen, ..
                        }) => break,
                        // consume trailing commas..
                        Some(Token { kind: TokenType::Comma, .. }) => {
                            next = self.lexer.next();
                        }
                        Some(Token {
                            kind: TokenType::Identifier | TokenType::NullableIdentifier | TokenType::Variable,
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
                            t @ Token {
                                kind: TokenType::Identifier | TokenType::NullableIdentifier, ..
                            }
                        ) => type_hint = Some(t.slice.to_string()),
                        Some(t @ Token { kind: TokenType::Variable, .. }) => {
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

                    let next = self.lexer.peek();
                    let mut default = None;

                    if matches!(next, Some(Token { kind: TokenType::Equals, .. })) {
                        self.lexer.next();

                        default = Some(self.parse_expression(0, None)?);
                    }

                    parameters.push(FunctionParameter::new(name, type_hint, default))
                }

                let mut return_type_hint = None;
                let next = self.lexer.next();

                if matches!(next, Some(Token { kind: TokenType::Colon, .. })) {
                    let return_type_token = self.expect_token(TokenType::Identifier, "")?;

                    return_type_hint = Some(return_type_token.slice.to_string());

                    self.expect_token(TokenType::DoubleArrow, "{")?;
                } else if next.is_some()
                    && !matches!(
                        next,
                        Some(Token {
                            kind: TokenType::DoubleArrow,
                            ..
                        })
                    )
                {
                    let next = next.unwrap();

                    return Err(ParserError::ExpectedToken {
                        expected_type: TokenType::DoubleArrow,
                        expected_slice: "=>",
                        got_type: next.kind,
                        got_slice: next.slice,
                    });
                }

                let expression = self.parse_expression(0, None)?;

                Expression::Closure(Function::new(
                    None,
                    parameters, 
                    vec![Statement::Expression(expression)],
                    return_type_hint,
                    Vec::new(), 
                    Some(ClosureType::Short)
                ))
            },
            TokenType::Function => {
                self.expect_token(TokenType::LeftParen, "(")?;

                let mut parameters: Vec<FunctionParameter> = Vec::new();

                loop {
                    let mut next = self.lexer.next();

                    println!("{:?}", next);

                    match next {
                        // break when finding a ), no more parameters
                        Some(Token {
                            kind: TokenType::RightParen, ..
                        }) => break,
                        // consume trailing commas..
                        Some(Token { kind: TokenType::Comma, .. }) => {
                            next = self.lexer.next();
                        }
                        Some(Token {
                            kind: TokenType::Identifier | TokenType::NullableIdentifier | TokenType::Variable,
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
                            t @ Token {
                                kind: TokenType::Identifier | TokenType::NullableIdentifier, ..
                            }
                        ) => type_hint = Some(t.slice.to_string()),
                        Some(t @ Token { kind: TokenType::Variable, .. }) => {
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

                    let next = self.lexer.peek();
                    let mut default = None;

                    if matches!(next, Some(Token { kind: TokenType::Equals, .. })) {
                        self.lexer.next();

                        default = Some(self.parse_expression(0, None)?);
                    }

                    parameters.push(FunctionParameter::new(name, type_hint, default))
                }

                let mut return_type_hint = None;
                let next = self.lexer.next();

                if matches!(next, Some(Token { kind: TokenType::Colon, .. })) {
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
                            kind: TokenType::RightBrace, ..
                        }) => break,
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let statement = self.match_token(next.unwrap())?;

                            body.push(statement);
                        }
                    }
                }

                Expression::Closure(Function::new(None, parameters, body, return_type_hint, Vec::new(), Some(ClosureType::Long)))
            },
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
            TokenType::LeftParen => {
                let expression = self.parse_expression(0, None)?;

                self.expect_token(TokenType::RightParen, ")")?;

                expression
            },
            TokenType::LeftBracket => {
                let mut items = Vec::new();
                let mut counter = 0;

                loop {
                    let next = self.lexer.peek();

                    match next {
                        Some(Token {
                            kind: TokenType::RightBracket, ..
                        }) => {
                            self.lexer.next();

                            break;
                        }
                        Some(Token { kind: TokenType::Comma, .. }) => {
                            self.lexer.next();

                            continue;
                        }
                        None => return Err(ParserError::UnexpectedEndOfFile),
                        _ => {
                            let expression = self.parse_expression(0, None)?;

                            match expression {
                                Expression::ArrayItem { ref key, .. } => {
                                    match **key {
                                        Expression::Integer(i) => counter = i + 1,
                                        Expression::Float(f) => counter = (f as i64) + 1,
                                        _ => (),
                                    }

                                    items.push(expression)
                                }
                                _ => {
                                    let key = Expression::Integer(counter.clone());

                                    items.push(Expression::ArrayItem {
                                        key: Box::new(key),
                                        value: Box::new(expression),
                                    });

                                    counter += 1
                                }
                            }
                        }
                    }
                }

                Expression::Array(items)
            }
            TokenType::Identifier | TokenType::NullableIdentifier => {
                match self.lexer.clone().next() {
                    Some(Token {
                        kind: TokenType::Variable,
                        slice,
                        ..
                    }) => {
                        let mut buffer = slice.to_string();
                        // remove the $
                        buffer.remove(0);

                        self.lexer.next();

                        Expression::TypedVariable(next.slice.to_owned(), buffer)
                    }
                    _ => Expression::Identifier(next.slice.to_owned()),
                }
            },
            TokenType::Minus => {
                let maybe_bp = BindingPower::prefix(TokenType::Minus);

                if maybe_bp.is_none() {
                    return Err(ParserError::Unknown);
                }

                let ((), rbp) = maybe_bp.unwrap();

                let rhs = self.parse_expression(rbp, None)?;

                Expression::Unary(Box::new(rhs))
            },
            TokenType::Not => {
                let maybe_bp = BindingPower::prefix(TokenType::Not);

                if maybe_bp.is_none() {
                    return Err(ParserError::Unknown);
                }

                let ((), rbp) = maybe_bp.unwrap();

                let rhs = self.parse_expression(rbp, None)?;

                Expression::Negate(Box::new(rhs))
            },
            TokenType::BitwiseNot => {
                let maybe_bp = BindingPower::prefix(TokenType::BitwiseNot);

                if maybe_bp.is_none() {
                    return Err(ParserError::Unknown);
                }

                let ((), rbp) = maybe_bp.unwrap();

                let rhs = self.parse_expression(rbp, None)?;

                Expression::BitwiseNot(Box::new(rhs))
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
                    TokenType::Arrow => {
                        let next = self.lexer.next();

                        match next {
                            Some(t @ Token { kind: TokenType::Identifier, .. }) => {
                                Expression::PropertyAccess(Box::new(lhs), Box::new(Expression::Identifier(t.slice.to_owned())))
                            },
                            None => return Err(ParserError::UnexpectedEndOfFile),
                            _ => {
                                let t = next.unwrap();
                                
                                return Err(ParserError::UnexpectedToken(t.kind, t.slice))
                            }
                        }
                    },
                    TokenType::LeftBracket => {
                        let next = self.lexer.next();

                        let expression = match next {
                            Some(Token {
                                kind: TokenType::RightBracket, ..
                            }) => None,
                            None => return Err(ParserError::UnexpectedEndOfFile),
                            _ => {
                                let index = self.parse_expression(0, next)?;

                                self.expect_token(TokenType::RightBracket, "]")?;

                                Some(Box::new(index))
                            }
                        };

                        Expression::ArrayAccess(Box::new(lhs.clone()), expression)
                    }
                    TokenType::LeftParen => {
                        let mut args = Vec::new();

                        loop {
                            let next = self.lexer.next();

                            let token = match next {
                                Some(t) => t,
                                None => return Err(ParserError::UnexpectedEndOfFile),
                            };

                            match token.kind {
                                TokenType::RightParen => break,
                                TokenType::Comma => {
                                    if args.is_empty() {
                                        return Err(ParserError::UnexpectedToken(TokenType::Comma, ","))
                                    }

                                    continue
                                },
                                _ => {
                                    let expression = self.parse_expression(0, next)?;

                                    args.push(expression)
                                }
                            }
                        }

                        Expression::Call {
                            target: Box::new(lhs),
                            args: args,
                        }
                    }
                    _ => unreachable!(),
                };

                continue;
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

    fn expect_left_paren(&mut self) -> Result<Token, ParserError<'p>> {
        self.expect_token(TokenType::LeftParen, "(")
    }

    fn expect_right_paren(&mut self) -> Result<Token, ParserError<'p>> {
        self.expect_token(TokenType::RightParen, ")")
    }

    fn expect_left_brace(&mut self) -> Result<Token, ParserError<'p>> {
        self.expect_token(TokenType::LeftBrace, "{")
    }

    fn expect_right_brace(&mut self) -> Result<Token, ParserError<'p>> {
        self.expect_token(TokenType::RightBrace, "}")
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
