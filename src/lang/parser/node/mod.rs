use std::fmt::Display;

use crate::lang::lexer::token::Token;

pub mod bin_op_node;
pub mod unary_node;
pub mod string_node;
pub mod number_node;
pub mod bool_node;
pub mod void_node;

// todo: add macro to auto generate visitor methods and get_token methods for all nodes (and also probably display methods)
pub trait Node: Display {
    fn get_token(&self) -> &Token;
    fn visit(&self, visitor: ());
}