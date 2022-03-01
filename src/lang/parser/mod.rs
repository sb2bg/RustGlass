use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;
use crate::lang::parser::node::{BinOpNode, Node};
use crate::Lexer;

pub mod node;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
    current: Option<&'a Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            index: 0,
            current: None,
        }
    }

    pub fn parse(&mut self) -> () {
        ()
    }
}