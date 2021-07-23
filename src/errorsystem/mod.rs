use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;

pub mod error_type;

pub fn dispatch_error(error: ErrorType, pos: Option<Position>) {
    eprintln!("\n\tFatal exception during runtime -> \"{}\"{}", error.to_string(), match pos {
        Some(unwrapped) => format!(" at {}", unwrapped.to_string()),
        None => String::new()
    });

    std::process::exit(1);
}