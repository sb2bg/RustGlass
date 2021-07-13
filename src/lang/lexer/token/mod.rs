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

    pub fn get_type(&self) -> TokenType {
        return self.token_type;
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return format_args!("{}{}", self.token_type.to_string(), if self.value.is_empty() { String::new() } else { format_args!("({})", self.value).to_string() }).to_string();
    }
}