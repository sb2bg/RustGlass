#![feature(type_ascription, backtrace)]

use std::{fs, panic};
use std::backtrace::Backtrace;
use std::time::Instant;

use clap::{App, Arg};
use git_version::git_version;

use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::Lexer;
use crate::lang::parser::Parser;
use crate::lang::REPL;

mod errorsystem;
mod lang;

fn main() {
    panic::set_hook(Box::new(|info| {
        let backtrace = if cfg!(debug_assertions) { // only print backtrace in debug mode
            Some(Backtrace::capture())
        } else {
            None
        };

        dispatch_error!(ErrorType::Fatal(format!("{}, Version: {}, Revision: {}", info, clap::crate_version!(), git_version!()), backtrace));
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
            let mut repl = REPL::new();
            repl.run();
            return;
        }
    }

    // todo: pass this to lang instead to be handled

    let src = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            dispatch_error!(ErrorType::UnknownFile(filename));
        }
    };

    let src = src.as_str();

    if debugging || token_debug {
        let mut debug = Lexer::new(filename, src);

        let start = Instant::now();
        let tokens = debug.lex();
        let end = Instant::now();

        if token_debug {
            for token in tokens {
                println!("{}", token);
            }
        }

        if debugging {
            let nanos = end.duration_since(start).as_nanos();
            println!("Lexing took {} nanos, {} millis", nanos, nanos as f64 / 1_000_000f64);
        }
    }

    let mut lexer = Lexer::new(filename, src);
    let mut parser = Parser::new(lexer.lex());
    let parsed = parser.parse();

    if debugging {
        println!("{}", parsed);
    }
}