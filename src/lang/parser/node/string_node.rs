use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::parser::node::Node;

pub struct StringNode {
    value: String,
}

impl StringNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for StringNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String({})", self.value)
    }
}