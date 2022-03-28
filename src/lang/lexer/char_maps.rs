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
        m.insert("=", TokenType::Equal);
        m.insert("%", TokenType::Mod);
        m.insert("**", TokenType::Pow);
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
        m.insert("==", TokenType::EqualEqual);
        m.insert("**=", TokenType::PowEquals);
        m.insert("void", TokenType::Void);
        m.insert("print", TokenType::Print);
        m.insert("println", TokenType::Println);
        m.insert("true", TokenType::True);
        m.insert("false", TokenType::False);
        m.insert("func", TokenType::Func);
        m.insert("end", TokenType::End);
        m.insert("list", TokenType::List);
        m.insert("dict", TokenType::Dict);
        m.insert("function", TokenType::Function);
        m.insert("in", TokenType::In);
        m.insert("if", TokenType::If);
        m.insert("is", TokenType::Is);
        m.insert("bool", TokenType::Bool);
        m.insert("num", TokenType::Num);
        m.insert("!=", TokenType::NotEqual);
        m.insert("or", TokenType::Or);
        m.insert("not", TokenType::Not);
        m.insert("str", TokenType::Str);
        m.insert("typeof", TokenType::Typeof);
        m
    };
}

lazy_static! {
    // the escape codes which can be used in strings
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

lazy_static! {
    // used to increment and decrement a wrap counter for implicit line joining
    static ref SINGLE_MAP: HashMap<char, (TokenType, Option<bool>)> = {
        let mut m = HashMap::new();
        m.insert('(', (TokenType::Lparen, Some(true)));
        m.insert(')', (TokenType::Rparen, Some(false)));
        m.insert('[', (TokenType::Lbracket, Some(true)));
        m.insert(']', (TokenType::Rbracket, Some(false)));
        m.insert('{', (TokenType::Lbracket, Some(true)));
        m.insert('}', (TokenType::Rbrace, Some(false)));
        m.insert(',', (TokenType::Comma, None));
        m.insert(':', (TokenType::Colon, None));
        m.insert('.', (TokenType::Period, None));
        m
    };
}

pub fn get_token<'a, S: Into<String>>(value: S) -> Option<&'a TokenType> {
    TOKEN_MAP.get(&*value.into())
}

pub fn get_esc<'a>(value: char) -> Option<&'a char> {
    ESC_MAP.get(&value)
}

pub fn get_single<'a>(value: char) -> Option<&'a (TokenType, Option<bool>)> {
    SINGLE_MAP.get(&value)
}

pub fn is_single(value: char) -> bool {
    SINGLE_MAP.contains_key(&value)
}