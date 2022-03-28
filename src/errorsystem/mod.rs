use crate::ErrorType;
use crate::lang::lexer::position::Position;

pub mod error_type;

#[macro_export]
macro_rules! dispatch_error {
    ($error: expr, $pos: expr) => {
        eprintln!("\n\tFatal exception during runtime -> \"{} at {}\"", $error, $pos);
        std::process::exit(1); // This is to make the compiler happy (avoid mismatched arms)
    };

    ($error: expr) => {
        eprintln!("\n\tFatal exception during runtime -> \"{}\"", $error);
        std::process::exit(1); // This is to make the compiler happy (avoid mismatched arms)
    };
}

pub struct ErrorHandler {
    in_catch_block: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self { in_catch_block: false }
    }

    // if the error occurs in a catch block, we don't want to print the error message, but rather
    // give the error to the catch block and let it handle it
    pub fn handle_runtime_error<'a>(&mut self, error: ErrorType<'a>, pos: Position) -> ErrorType<'a> {
        if self.in_catch_block {
            error
        } else {
            dispatch_error!(error, pos);
        }
    }
}