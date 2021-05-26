use thiserror::Error;
use std::convert::From;
use std::num::ParseIntError;
use std::num::ParseFloatError;

#[derive(Debug, Error)]
pub enum ParserError {

    #[error("Failed to convert a numeric string into an integer.")]
    IntegerParserError,
    #[error("Failed to convert a numeric string into a float.")]
    FloatParserError,

    #[error("Unknown parser error.")]
    Unknown,
}

impl From<ParseIntError> for ParserError {
    fn from(_: ParseIntError) -> Self {
        Self::IntegerParserError
    }
}

impl From<ParseFloatError> for ParserError {
    fn from(_: ParseFloatError) -> Self {
        Self::FloatParserError
    }
}