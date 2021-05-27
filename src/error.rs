use crate::Statement;
use crate::Function;

use std::convert::From;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use thiserror::Error;
use tusk_lexer::TokenType;

#[derive(Debug, Error)]
pub enum ParserError<'pe> {
    #[error("Invalid file type. Could not find opening PHP tag.")]
    InvalidFileType,

    #[error("Failed to convert a numeric string into an integer.")]
    IntegerParserError,
    #[error("Failed to convert a numeric string into a float.")]
    FloatParserError,

    #[error("Unexpected statement {0:?}.")]
    UnexpectedStatement(Statement),

    #[error("The method `{0}` has already been defined.")]
    MethodAlreadyExists(String),

    #[error("Expected token {expected_type:?} ({expected_slice}), got {got_type:?} ({got_slice})")]
    ExpectedToken {
        expected_type: TokenType,
        expected_slice: &'pe str,
        got_type: TokenType,
        got_slice: &'pe str,
    },

    #[error("Unexpected token {0:?} ({1}).")]
    UnexpectedToken(TokenType, &'pe str),

    #[error("Unexpected end of file.")]
    UnexpectedEndOfFile,

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
