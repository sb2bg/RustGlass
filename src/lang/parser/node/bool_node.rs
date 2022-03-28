use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::parser::node::Node;

pub struct BoolNode {
    value: bool,
}

impl BoolNode {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for BoolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BOol({})", self.value)
    }
}