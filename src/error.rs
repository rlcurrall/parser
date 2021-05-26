use thiserror::Error;
use std::convert::From;
use std::num::ParseIntError;
use std::num::ParseFloatError;
use tusk_lexer::TokenType;

#[derive(Debug, Error)]
pub enum ParserError<'pe> {
    #[error("Invalid file type. Could not find opening PHP tag.")]
    InvalidFileType,

    #[error("Failed to convert a numeric string into an integer.")]
    IntegerParserError,
    #[error("Failed to convert a numeric string into a float.")]
    FloatParserError,

    #[error("Unexpected token {0:?} ({1})")]
    UnexpectedToken(TokenType<'pe>, &'pe str),

    #[error("Unknown parser error.")]
    Unknown,
}

impl<'pe> From<ParseIntError> for ParserError<'pe> {
    fn from(_: ParseIntError) -> Self {
        Self::IntegerParserError
    }
}

impl<'pe> From<ParseFloatError> for ParserError<'pe> {
    fn from(_: ParseFloatError) -> Self {
        Self::FloatParserError
    }
}