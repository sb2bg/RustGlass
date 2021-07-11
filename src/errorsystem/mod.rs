use std::borrow::Cow;

use crate::errorsystem::error_type::{error_to_string, ErrorType};
use crate::lang::lexer::position::Position;
use crate::lang::lexer::position::fragment::PositionFragment;

pub mod error_type;

const RED: &str = "\x1b[31m";

pub fn dispatch_error(error: ErrorType, args: Option<&[&str]>, pos: Option<PositionFragment>) {
    eprintln!("{}Fatal exception during runtime -> \"{}\" at {}", RED, format(error_to_string(error), args), match pos {
        Some(unwrapped) => unwrapped.to_string(),
        None => String::from("unknown location")
    });
    std::process::exit(0);
}

fn format(msg: &'static str, args: Option<&[&str]>) -> String {
    return match args {
        Some(args) => {
            let mut temp = Cow::from(msg);

            for arg in args {
                temp = temp.replacen("{}", arg, 1).into();
            }

            return temp.to_string();
        }
        None => String::from(msg)
    };
}