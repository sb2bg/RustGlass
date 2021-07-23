use std::fs;
use std::ops::Sub;
use std::time::SystemTime;

use clap::{App, Arg};

use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::Lexer;
use crate::lang::lexer::position::Position;

mod errorsystem;
mod lang;

fn main() {
    let matches = App::new("RustGlass")
        .version("1.0.0")
        .author("Sullivan B")
        .about("Dynamically typed language written in RustLang")
        .arg(Arg::with_name("filename")
            .index(1)
            .takes_value(true)
            .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let src = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            dispatch_error(ErrorType::UnknownFile(filename), None);
            panic!(); // (not called) avoid incompatible arm type error
        }
    };
    let src = src.as_str();
    let mut lexer = Lexer::new(filename, src);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{}", token.to_string())
    }
}