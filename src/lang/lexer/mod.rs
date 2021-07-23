use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::errorsystem::error_type::ErrorType::UnknownChar;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;

mod token;
mod char_maps;
pub mod position;

pub struct Lexer<'a> {
    value: Vec<char>,
    source: &'a str,
    index: usize,
    position: Position<'a>,
    current: char,
}

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let first = if chars.len() <= 0 { '\0' } else { *chars.get(0).unwrap() };

        Self {
            source,
            value: chars,
            index: 0,
            current: first,
            position: Position::new(filename, source),
        }
    }

    // todo - implicit line joins (python inspired <3)
    pub fn lex(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        while !self.is_done() {
            if self.current.is_ascii_digit() {
                tokens.push(self.consume_number());
            } else if self.is_newline() {
                if tokens.get(tokens.len() - 1).unwrap().get_type() != TokenType::Newline { tokens.push(self.consume_newline()) } else { self.advance() };
            } else if self.current.is_whitespace() {
                self.advance();
            } else if self.is_operator() {
                tokens.push(self.consume_operator());
            } else if self.is_single() {
                tokens.push(self.consume_single());
            } else if self.is_quote() {
                tokens.push(self.consume_string());
            } else if self.is_comment() {
                self.consume_comment();
            } else {
                tokens.push(self.consume_identifier());
            }
        }

        tokens
    }

    fn consume_newline(&mut self) -> Token<'a> {
        self.advance();
        Token::new(TokenType::Newline, None, self.position)
    }

    fn consume_number(&mut self) -> Token<'a> {
        let start = self.position.clone();
        let mut buffer = String::new();
        let mut dec = false;

        while !self.is_done() && self.current.is_ascii_digit() || self.current == '.' {
            if self.current == '.' {
                if dec {
                    dispatch_error(ErrorType::DoubleDecimal, Some(self.position));
                }
                dec = true;
            }

            buffer.push(self.current);
            self.advance();
        }

        if buffer.ends_with('.') {
            dispatch_error(ErrorType::DecimalEnding, Some(self.position));
        }

        Token::new(TokenType::Number, Some(buffer), start)
    }

    fn consume_operator(&mut self) -> Token<'a> {
        let start = self.position.clone();
        let mut buffer = String::new();

        while !self.is_done() && self.is_operator()
        {
            buffer.push(self.current);
            self.advance();
        }

        match char_maps::get_token(buffer) {
            Ok(operator) => Token::new(operator, None, start),
            Err(value) => {
                dispatch_error(ErrorType::InvalidOperator(value.as_str()), Some(self.position));
                panic!(); // (not called) avoid incompatible arm type error
            }
        }
    }

    // todo - test strings and all that fun stuff!
    fn consume_string(&mut self) -> Token<'a> {
        let start = self.position.clone();
        self.advance();
        let mut buffer = String::new();
        let mut esc = false;

        while !self.is_done() && !self.is_quote() {
            if esc {
                match char_maps::get_esc(self.current) {
                    Ok(escaped) => buffer.push(escaped),
                    Err(unknown) => {
                        dispatch_error(ErrorType::UnknownEscapeSequence(unknown), Some(self.position));
                        panic!(); // (not called) avoid incompatible arm type error
                    }
                }
                esc = false;
            } else if self.current == '\\' {
                esc = true;
            } else {
                buffer.push(self.current);
            }

            self.advance();
        }

        if !self.is_quote() {
            dispatch_error(ErrorType::UnclosedString, Some(self.position));
        }

        self.advance();
        Token::new(TokenType::String, Some(buffer), start)
    }

    fn consume_single(&mut self) -> Token<'a> {
        let start = self.position.clone();

        let token = match char_maps::get_token(String::from(self.current)) {
            Ok(token) => token,
            Err(single) => {
                dispatch_error(ErrorType::UnknownChar(single.chars().nth(0).unwrap()), Some(self.position));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        self.advance();
        Token::new(token, None, start)
    }

    fn consume_comment(&mut self) {
        while !self.is_done() && !self.is_newline() {
            self.advance();
        }
        self.advance();
    }

    fn consume_identifier(&mut self) -> Token<'a> {
        let start = self.position.clone();
        let mut buffer = String::new();

        while !self.is_done() && self.current == '_' || self.current.is_ascii_digit() || self.current.is_ascii_alphabetic()
        {
            buffer.push(self.current);
            self.advance();
        }

        if buffer.len() < 1 {
            dispatch_error(UnknownChar(self.current), Some(self.position));
        }

        match char_maps::get_token(buffer) {
            Ok(token) => Token::new(token, None, start),
            Err(value) => Token::new(TokenType::Identifier, Some(value), start)
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current = if self.is_done() { '\0' } else { self.value[self.index] };
        self.position.advance(self.is_newline());
    }

    fn is_done(&self) -> bool {
        self.index >= self.value.len() || self.current == '\0'
    }

    fn is_newline(&self) -> bool {
        "\n;".contains(self.current)
    }

    fn is_operator(&self) -> bool {
        "+-*/%<>=!".contains(self.current)
    }

    fn is_single(&self) -> bool {
        "/,.()[]{}:".contains(self.current)
    }

    fn is_quote(&self) -> bool {
        self.current == '"'
    }

    fn is_comment(&self) -> bool {
        self.current == '#'
    }
}