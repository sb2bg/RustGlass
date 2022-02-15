use std::fmt::Display;

use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::token_type::TokenType;

pub mod token_type;

pub struct Token<'a> {
    token_type: TokenType,
    value: String,
    pos: Position<'a>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, value: Option<String>, pos: Position<'a>) -> Self {
        Token {
            token_type,
            value: match value {
                Some(value) => value,
                None => String::new()
            },
            pos,
        }
    }

    pub fn get_type(&self) -> TokenType {
        return self.token_type;
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type: &str = self.token_type.into();
        write!(f, "{}{}", token_type, if self.value.is_empty() { String::new() } else { format!("({})", self.value) })
    }
}