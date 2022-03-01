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

