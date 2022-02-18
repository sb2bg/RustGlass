use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::token::Token;
use crate::Lexer;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
    current: &'a Token<'a>,
}

impl<'a> Parser<'a> {}