pub enum ErrorType {
    GenericError,
    DoubleDecimal,
    DecimalEnding,
    UnknownFile,
    DivisionByZero,
    UnknownChar,
    UnclosedString,
    UnknownEscapeSequence,
    UnexpectedExpression,
    InvalidInversion,
    InvalidOperator,
    NoOperatorDefinition,
    InvalidIndex,
    NoDefiningScope,
    OutOfBounds,
    UnexpectedType,
    InvalidCall,
    UnexpectedArgCount,
    InvalidIteration,
    UnknownKeyword,

}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        return String::from(match self {
            ErrorType::GenericError => "Unknown error occurred during the '{}' process",
            ErrorType::UnknownFile => "Couldn't find file '{}'",
            ErrorType::DoubleDecimal => "Cannot have two decimals in a number",
            ErrorType::DecimalEnding => "Number cannot end in a decimal",
            ErrorType::DivisionByZero => "Cannot divide by zero",
            ErrorType::UnknownChar => "Unknown character '{}'",
            ErrorType::UnclosedString => "Unclosed string",
            ErrorType::UnknownEscapeSequence => "Unknown escape sequence '{}'",
            ErrorType::UnexpectedExpression => "Expected '{}'",
            ErrorType::InvalidInversion => "Cannot invert value of type '{}'",
            ErrorType::InvalidOperator => "Invalid operator '{}'",
            ErrorType::NoOperatorDefinition => "No operator definition on type '{}'",
            ErrorType::InvalidIndex => "Invalid index '{}'",
            ErrorType::NoDefiningScope => "Variable '{}' undefined",
            ErrorType::OutOfBounds => "Index {} out of bounds for range {}",
            ErrorType::UnexpectedType => "Expected type '{}' but got '{}'",
            ErrorType::InvalidCall => "Cannot call type '{}'",
            ErrorType::UnexpectedArgCount => "Expected {} args, instead got {}",
            ErrorType::InvalidIteration => "Cannot iterate over type '{}'",
            ErrorType::UnknownKeyword => "Unknown keyword '{}'",
        });
    }
}