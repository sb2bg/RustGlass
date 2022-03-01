use std::any::Any;

use crate::lang::lexer::token::Token;

pub trait Node {
    fn get_token(&self) -> &Token;
    fn visit(&self, visitor: ());
}

pub struct BinOpNode<'a> {
    pub token: &'a Token<'a>,
    pub value: (&'a dyn Node, &'a dyn Node),
}