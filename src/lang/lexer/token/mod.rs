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

impl ToString for Token<'_> {
    fn to_string(&self) -> String {
        format!("{}{}", self.token_type.to_string(), if self.value.is_empty() { String::new() } else { format_args!("({})", self.value).to_string() })
    }
}