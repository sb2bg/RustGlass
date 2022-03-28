use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::parser::node::Node;

pub struct VoidNode {}

impl VoidNode {
    pub fn new() -> VoidNode {
        VoidNode {}
    }
}

impl Node for VoidNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for VoidNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Void")
    }
}