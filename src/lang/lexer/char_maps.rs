use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::lang::lexer::token::token_type::TokenType;

lazy_static! {
    static ref CHAR_MAP: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("+", TokenType::Plus);
        m.insert("-", TokenType::Minus);
        m.insert("*", TokenType::Times);
        m.insert("/", TokenType::Divide);
        m.insert("%", TokenType::Mod);
        m.insert(">", TokenType::GreaterThan);
        m.insert(">=", TokenType::GreaterThanEqual);
        m.insert("<", TokenType::LessThan);
        m.insert("<=", TokenType::LessThanEqual);
        m.insert("=>", TokenType::Lambda);
        m.insert("+=", TokenType::PlusEquals);
        m.insert("-=", TokenType::MinusEquals);
        m.insert("*=", TokenType::TimesEquals);
        m.insert("/=", TokenType::DivideEquals);
        m.insert("%=", TokenType::ModEquals);
        return m;
    };
}

pub fn get(value: String) -> Result<TokenType, String> {
    return match CHAR_MAP.get(value.as_str()) {
        Some(result) => Ok(*result),
        None => Err(value)
    };
}