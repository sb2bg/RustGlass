use std::borrow::Cow;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::fragment::PositionFragment;

pub mod error_type;

pub fn dispatch_error(error: ErrorType, args: Option<&[String]>, pos: Option<PositionFragment>) {
    if !print_red(format_args!("\n\tFatal exception during runtime -> \"{}\"{}", format(error.to_string(), args), match pos {
        Some(unwrapped) => format_args!(" at {}", unwrapped.to_string()).to_string(),
        None => String::new()
    }).to_string()).is_ok() {
        eprintln!("Failed to write glass error message");
    } else {
        if !print_clear().is_ok() {
            eprintln!("Failed to default console colors");
        }
    }

    std::process::exit(1);
}

fn format(msg: String, args: Option<&[String]>) -> String {
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

fn print_red(msg: String) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    return writeln!(&mut stdout, "{}", msg);
}

fn print_clear() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&ColorSpec::default())?;
    return writeln!(&mut stdout);
}