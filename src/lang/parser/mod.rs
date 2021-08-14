use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::token::Token;
use crate::lang::parser::node::Node;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
    current: &'a Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>, filename: &'a str) -> Self {
        if tokens.len() < 1 {
            dispatch_error(ErrorType::EmptyFile(filename), None)
        }

        let first = tokens.get(0).unwrap();

        Self { tokens, index: 0, current: first }
    }

    fn statements(&self) {}
}