use std::{fs, panic};
use std::time::Instant;

use clap::{App, Arg};

use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::Lexer;

mod errorsystem;
mod lang;

fn main() {
    panic::set_hook(Box::new(|info| {
        dispatch_error(ErrorType::Fatal(format!("{}, Version: {}", info, clap::crate_version!())), None);
    }));

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("filename")
            .index(1)
            .takes_value(true)
            .help("The name of the file you want to run"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .help("Prints lexing, parsing and interpreting debug info"))
        .arg(Arg::with_name("tokens")
            .long("tokens")
            .short("t")
            .help("Shows the tokens that were lexed"))
        .get_matches();

    let filename;
    let debugging = matches.is_present("debug");
    let token_debug = matches.is_present("tokens");

    match matches.value_of("filename") {
        Some(value) => { filename = value; }
        None => {
            let mut reader = String::new();
            let stdin = std::io::stdin();

            while stdin.read_line(&mut reader).is_ok() {
                println!("echo: {}", reader);
            }

            return;
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
    let mut debug = Lexer::new(filename, src);
    let start = Instant::now();

    while let Some(token) = debug.next() {
        if token_debug { // prints token list if flag -t is included
            println!("{}", token.to_string());
        }
    }

    let end = Instant::now();

    // prints info if -d flag is included
    if debugging {
        let nanos = end.duration_since(start).as_nanos();
        println!("Lexing took {} nanos, {} millis", nanos, nanos as f64 / 1_000_000f64);
    }

    let mut lexer = Lexer::new(filename, src);
    // todo -> pass to parser
}