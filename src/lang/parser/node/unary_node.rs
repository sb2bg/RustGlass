use std::fmt::Display;

use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;
use crate::lang::parser::node::Node;

pub struct UnaryNode {
    op: TokenType,
    expr: Box<dyn Node>,
}

impl UnaryNode {
    pub fn new(op: TokenType, expr: Box<dyn Node>) -> Self {
        UnaryNode { op, expr }
    }
}

impl Node for UnaryNode {
    fn get_token(&self) -> &Token {
        todo!()
    }

    fn visit(&self, visitor: ()) {
        todo!()
    }
}

impl Display for UnaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary({} {})", self.op.into(): &str, self.expr)
    }
}