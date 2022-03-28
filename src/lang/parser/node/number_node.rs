use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::parser::node::Node;

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        NumberNode { value }
    }
}

impl Node for NumberNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for NumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number({})", self.value)
    }
}