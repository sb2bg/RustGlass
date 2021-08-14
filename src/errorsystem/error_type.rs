pub enum ErrorType<'a> {
    GenericError(&'a str),
    DoubleDecimal,
    DecimalEnding,
    UnknownFile(&'a str),
    DivisionByZero,
    UnknownChar(char),
    UnclosedString,
    UnknownEscapeSequence(char),
    UnexpectedExpression(&'a str),
    InvalidInversion(&'a str),
    InvalidOperator(&'a str),
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
}

impl ToString for ErrorType<'_> {
    fn to_string(&self) -> String {
        match self {
            ErrorType::GenericError(a1) => format!("Unknown error occurred during the '{}' process", a1),
            ErrorType::UnknownFile(a1) => format!("Couldn't find file '{}'", a1),
            ErrorType::DoubleDecimal => String::from("Cannot have two decimals in a number"),
            ErrorType::DecimalEnding => String::from("Number cannot end in a decimal"),
            ErrorType::DivisionByZero => String::from("Cannot divide by zero"),
            ErrorType::UnknownChar(a1) => format!("Unknown character '{}' encountered", a1),
            ErrorType::UnclosedString => String::from("Unclosed string"),
            ErrorType::UnknownEscapeSequence(a1) => format!("Unknown escape sequence '{}'", a1),
            ErrorType::UnexpectedExpression(a1) => format!("Expected '{}'", a1),
            ErrorType::InvalidInversion(a1) => format!("Cannot invert value of type '{}'", a1),
            ErrorType::InvalidOperator(a1) => format!("Invalid operator '{}'", a1),
            ErrorType::NoOperatorDefinition(a1) => format!("No operator definition on type '{}'", a1),
            ErrorType::InvalidIndex(a1) => format!("Invalid index '{}'", a1),
            ErrorType::NoDefiningScope(a1) => format!("Variable '{}' undefined", a1),
            ErrorType::OutOfBounds(a1, a2) => format!("Index {} out of bounds for range {}", a1, a2),
            ErrorType::UnexpectedType(a1, a2) => format!("Expected type '{}' but got '{}'", a1, a2),
            ErrorType::InvalidCall(a1) => format!("Cannot call type '{}'", a1),
            ErrorType::UnexpectedArgCount(a1, a2) => format!("Expected {} args, instead got {}", a1, a2),
            ErrorType::InvalidIteration(a1) => format!("Cannot iterate over type '{}'", a1),
            ErrorType::UnknownKeyword(a1) => format!("Unknown keyword '{}'", a1),
            ErrorType::EmptyFile(a1) => format!("Cannot parse empty file '{}'", a1)
        }
    }
}