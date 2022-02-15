use std::io;
use std::io::Write;

use crate::lang::interpreter::primitive::{Primitive, Type};

pub mod lexer;
pub mod parser;
pub mod interpreter;

const REPL_FILENAME: &str = "REPL";

pub struct REPL<'a> {
    interpreter: interpreter::Interpreter<'a>,
}

impl<'a> REPL<'a> {
    pub fn new() -> Self {
        REPL {
            interpreter: interpreter::Interpreter::new(None, REPL_FILENAME),
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let tokens = lexer::Lexer::new(REPL_FILENAME, input).lex();
            //let ast = parser::Parser::new(tokens).parse();
            //let result = self.interpreter.interpret(ast);
            let result = 4;
            println!("{}", result);
        }
    }
}