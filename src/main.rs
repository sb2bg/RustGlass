use std::fs;

use clap::{App, Arg};

use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::Lexer;

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

    // todo - pull filename from clap
    let filename = matches.value_of("filename").unwrap().to_string();

    let src = match read_file(String::from(filename.clone())) {
        Ok(res) => res,
        Err(_) => {
            dispatch_error(ErrorType::UnknownFile, Some(&[String::from(filename)]), None);
            panic!(); // (not called) avoid incompatible arm type error
        }
    };

    let mut lexer = Lexer::new(filename, src);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{}", token.to_string())
    }
}

fn read_file(file: String) -> Result<String, std::io::Error> {
    return match fs::read_to_string(file) {
        Ok(contents) => Ok(contents),
        Err(err) => Err(err)
    };
}