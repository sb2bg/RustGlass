use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;

pub mod token;
mod char_maps;
pub mod position;

pub struct Lexer<'a> {
    value: Vec<char>,
    index: usize,
    position: Position<'a>,
    current: char,
    filename: &'a str,
    wrap_count: usize, // usize instead of bool because of nested brackets, parentheses, or braces
}

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let first = if chars.len() <= 0 { '\0' } else { *chars.get(0).unwrap() };

        Self {
            filename,
            value: chars,
            index: 0,
            current: first,
            position: Position::new(filename, source),
            wrap_count: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        while !self.is_done() { // we have more tokens to consume
            if self.is_newline() {
                if self.get_last_token(&tokens) == TokenType::Newline || self.wrap_count > 0 { self.advance(); } else { tokens.push(self.consume_newline()); }; // implicit line joining
            } else if self.current.is_whitespace() {
                self.advance();
            } else if self.current.is_ascii_digit() {
                tokens.push(self.consume_number());
            } else if self.is_operator() {
                tokens.push(self.consume_operator());
            } else if self.single_check() {
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

    pub fn get_filename(&self) -> &'a str {
        self.filename
    }

    fn consume_newline(&mut self) -> Token<'a> {
        self.advance();
        Token::new(TokenType::Newline, None, self.position)
    }

    fn consume_number(&mut self) -> Token<'a> {
        let start = self.position;
        let mut buffer = String::new();
        let mut dec = false;

        while !self.is_done() && self.current.is_ascii_digit() || self.current == '.' {
            if self.current == '.' {
                if dec { // already had a decimal in the number
                    dispatch_error(ErrorType::DoubleDecimal, Some(self.position));
                }
                dec = true;
            }

            buffer.push(self.current);
            self.advance();
        }

        if buffer.ends_with('.') { // number can't end with a decimal
            dispatch_error(ErrorType::DecimalEnding, Some(self.position));
        }

        Token::new(TokenType::Number, Some(buffer), start)
    }

    fn consume_operator(&mut self) -> Token<'a> {
        let start = self.position;
        let mut buffer = String::new();

        while !self.is_done() && self.is_operator() {
            buffer.push(self.current);
            self.advance();
        }

        match char_maps::get_token(&buffer) {
            Some(operator) => Token::new(*operator, None, start),
            _ => {
                dispatch_error(ErrorType::InvalidOperator(buffer), Some(start));
                panic!(); // (not called) avoid incompatible arm type error
            }
        }
    }

    // todo - test strings and all that fun stuff!
    fn consume_string(&mut self) -> Token<'a> {
        let start = self.position;
        self.advance();
        let mut buffer = String::new();
        let mut esc = false;

        while !self.is_done() && !self.is_quote() {
            if esc {
                match char_maps::get_esc(self.current) {
                    Some(escaped) => buffer.push(*escaped),
                    None => {
                        dispatch_error(ErrorType::UnknownEscapeSequence(self.current), Some(self.position));
                        panic!(); // (not called) avoid incompatible arm type error
                    }
                }
                esc = false;
            } else if self.current == '\\' { // escape character
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
        let start = self.position;

        let &token_type = match char_maps::get_single(self.current) {
            Some((token_type, wrap)) => {
                if let Some(open) = wrap {
                    if *open {
                        self.wrap_count += 1;
                    } else if self.wrap_count > 0 {
                        self.wrap_count -= 1;
                    }
                }
                token_type
            }
            None => {
                dispatch_error(ErrorType::UnknownChar(self.current), Some(start));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        self.advance();
        Token::new(token_type, None, start)
    }

    fn consume_comment(&mut self) {
        while !self.is_done() && !self.is_newline() {
            self.advance();
        }
        self.advance();
    }

    fn consume_identifier(&mut self) -> Token<'a> {
        let start = self.position;
        let mut buffer = String::new();

        while !self.is_done() && self.current == '_' || self.current.is_ascii_digit() || self.current.is_ascii_alphabetic()
        {
            buffer.push(self.current);
            self.advance();
        }

        if buffer.len() < 1 {
            dispatch_error(ErrorType::UnknownChar(self.current), Some(self.position));
        }

        match char_maps::get_token(&buffer) {
            Some(&token) => Token::new(token, None, start),
            None => Token::new(TokenType::Identifier, Some(buffer), start)
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current = if self.is_done() { '\0' } else { self.value[self.index] };
        self.position.advance(self.is_newline());
    }

    fn get_last_token(&self, tokens: &Vec<Token>) -> TokenType {
        if tokens.len() > 0 { tokens.last().unwrap().get_type() } else { TokenType::Newline }
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

    fn single_check(&mut self) -> bool {
        char_maps::is_single(self.current)
    }

    fn is_quote(&self) -> bool {
        self.current == '"'
    }

    fn is_comment(&self) -> bool {
        self.current == '#'
    }
}