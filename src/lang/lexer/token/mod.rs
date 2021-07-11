use crate::lang::lexer::position::fragment::PositionFragment;
use crate::lang::lexer::token::token_type::TokenType;

pub mod token_type;

pub struct Token {
    token_type: TokenType,
    value: String,
    pos: PositionFragment,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, pos: PositionFragment) -> Self {
        return Token {
            token_type,
            value: match value {
                Some(value) => value,
                None => String::new()
            },
            pos,
        };
    }
}