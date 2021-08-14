use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::token::Token;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
    current: &'a Token<'a>,
}

impl<'a> Parser<'a> {
}

impl Iterator for Parser<'_> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}