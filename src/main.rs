use std::fs;
use std::time::SystemTime;

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
            .help("The name of the file you want to run"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .help("Prints lexing, parsing and interpreting debug info"))
        .get_matches();

    let filename;
    let debugging = matches.is_present("debug");

    match matches.value_of("filename") {
        Some(value) => { filename = value; }
        None => {
            // todo - not marked as todo!() because intellij isn't smart enough to know filename has to be initialized
            unimplemented!();
        }
    }

    let src = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            dispatch_error(ErrorType::UnknownFile(filename), None);
            panic!(); // (not called) avoid incompatible arm type error
        }
    };
    let src = src.as_str();
    let mut lexer = Lexer::new(filename, src);
    let start = SystemTime::now();
    let tokens = lexer.lex();
    let end = SystemTime::now();

    // prints info if -d flag is included
    if debugging {
        let nanos = end.duration_since(start).unwrap().as_nanos();
        println!("Lexing took {} nanos, {} millis", nanos, nanos as f64 / 1_000_000f64);
        println!("Total of {} tokens created", tokens.len());
        println!("{}", tokens.into_iter().map(|token| token.to_string() + ", ").collect::<String>());
    }
}