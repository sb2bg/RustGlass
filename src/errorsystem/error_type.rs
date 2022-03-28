use std::backtrace::Backtrace;
use std::fmt::Display;

use crate::lang::lexer::token::token_type::TokenType;

pub enum ErrorType<'a> {
    GenericError(&'a str),
    DoubleDecimal,
    DecimalEnding,
    UnknownFile(&'a str),
    DivisionByZero,
    UnknownChar(char),
    UnclosedString,
    UnknownEscapeSequence(char),
    UnexpectedExpression(TokenType, TokenType),
    InvalidInversion(&'a str),
    InvalidOperator(String),
    NoOperatorDefinition(&'a str),
    InvalidIndex(usize),
    NoDefiningScope(&'a str),
    OutOfBounds(usize, usize),
    UnexpectedType(&'a str, &'a str),
    InvalidCall(&'a str),
    UnexpectedArgCount(usize, usize),
    InvalidIteration(&'a str),
    UnknownKeyword(&'a str),
    EmptyFile(&'a str),
    ReachedEndOfFile,
    Fatal(String, Option<Backtrace>),
}

impl Display for ErrorType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::GenericError(a1) => write!(f, "Unknown error occurred during the '{}' process", a1),
            ErrorType::UnknownFile(a1) => write!(f, "File or directory '{}' was unable to be found", a1),
            ErrorType::DoubleDecimal => write!(f, "Cannot have two decimals in a number"),
            ErrorType::DecimalEnding => write!(f, "Number cannot end in a decimal"),
            ErrorType::DivisionByZero => write!(f, "Cannot divide by zero"),
            ErrorType::UnknownChar(a1) => write!(f, "Unknown character '{}' encountered", a1),
            ErrorType::UnclosedString => write!(f, "Unclosed string starting"),
            ErrorType::UnknownEscapeSequence(a1) => write!(f, "Escape sequence '{}' unknown", a1),
            ErrorType::UnexpectedExpression(a1, a2) => write!(f, "Expected {} but got {}", a1.into(): &str, a2.into(): &str),
            ErrorType::InvalidInversion(a1) => write!(f, "Type '{}' cannot be inverted", a1),
            ErrorType::InvalidOperator(a1) => write!(f, "Operator '{}' is invalid", a1),
            ErrorType::NoOperatorDefinition(a1) => write!(f, "Cannot use operator '{}' on this type", a1),
            ErrorType::InvalidIndex(a1) => write!(f, "Invalid index '{}'", a1),
            ErrorType::NoDefiningScope(a1) => write!(f, "Variable '{}' undefined", a1),
            ErrorType::OutOfBounds(a1, a2) => write!(f, "Index {} out of bounds for range {}", a1, a2),
            ErrorType::UnexpectedType(a1, a2) => write!(f, "Expected type '{}' but got '{}' instead", a1, a2),
            ErrorType::InvalidCall(a1) => write!(f, "Type '{}' cannot be called", a1),
            ErrorType::UnexpectedArgCount(a1, a2) => write!(f, "Expected {} args but got {}", a1, a2),
            ErrorType::InvalidIteration(a1) => write!(f, "Type '{}' is not iterable", a1),
            ErrorType::UnknownKeyword(a1) => write!(f, "Keyword '{}' unknown", a1),
            ErrorType::EmptyFile(a1) => write!(f, "File '{}' is empty", a1),
            ErrorType::ReachedEndOfFile => write!(f, "Unexpectedly reached end of file while parsing"),
            ErrorType::Fatal(a1, a2) => write!(f, "Glass fatal error: '{}' ->\n\t\tPlease report this crash at https://github.com/sb2bg/RustGlass/issues/new?template=glass-crash.md{}", a1, match a2 {
                Some(a2) => format!("\n\n\tStack trace ->\n\t{}", a2.to_string().trim_end().replace("\n", "\n\t")),
                None => String::new(),
            }),
        }
    }
}