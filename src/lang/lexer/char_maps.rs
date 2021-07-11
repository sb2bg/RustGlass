use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::lang::lexer::token::token_type::TokenType;

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, TokenType> = {
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
        m
    };
}

lazy_static! {
    // todo - test escape codes
    static ref ESC_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('a', '\x07');
        m.insert('b', '\x08');
        m.insert('f', '\x0C');
        m.insert('n', '\n');
        m.insert('t', '\t');
        m.insert('r', '\r');
        m.insert('\'', '\'');
        m.insert('"', '"');
        m.insert('\\', '\\');
        m
    };
}

pub fn get_token(value: String) -> Result<TokenType, String> {
    return match TOKEN_MAP.get(value.as_str()) {
        Some(result) => Ok(*result),
        None => Err(value)
    };
}

pub fn get_esc(value: char) -> Result<char, char> {
    return match ESC_MAP.get(&value) {
        Some(result) => Ok(*result),
        None => Err(value)
    };
}