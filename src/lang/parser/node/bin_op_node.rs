use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;
use crate::lang::parser::node::Node;

pub struct BinOpNode {
    op: TokenType,
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}


impl BinOpNode {
    pub fn new(op: TokenType, left: Box<dyn Node>, right: Box<dyn Node>) -> Self {
        Self { op, left, right }
    }
}

impl Node for BinOpNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for BinOpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BinOpNode ({} {} {})", self.left, self.op.into(): &str, self.right)
    }
}

