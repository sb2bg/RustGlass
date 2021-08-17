use std::iter::Peekable;
use std::str::Chars;

use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;

pub mod token;
pub mod position;
mod char_maps;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    position: Position<'a>,
    bracket_count: usize,
    newline: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        let chars = source.trim().chars().peekable();

        Self { source, chars, position: Position::new(filename, source), bracket_count: 0, newline: true }
    }

    fn consume_number(&mut self) -> Token<'a> {
        let start = self.position;
        let mut buffer = String::new();
        let mut dec = false;

        while let Some(&curr) = self.chars.peek() {
            if !(curr.is_ascii_digit() || curr == '.') { // break if no longer consuming a number
                break;
            }

            if curr == '.' {
                if dec { // already decimal in the number
                    dispatch_error(ErrorType::DoubleDecimal, Some(self.position));
                }

                dec = true; // note that we've hit a decimal
            }

            buffer.push(curr);
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

        while let Some(&curr) = self.chars.peek()
        {
            if !Self::is_operator(curr) {
                break;
            }

            buffer.push(curr);
            self.advance();
        }

        match char_maps::get_token(&buffer) {
            Some(operator) => Token::new(*operator, None, start),
            None => {
                dispatch_error(ErrorType::InvalidOperator(buffer), Some(start));
                panic!(); // (not called) avoid incompatible arm type error
            }
        }
    }

    fn consume_string(&mut self) -> Token<'a> {
        let start = self.position;
        self.advance();

        let mut buffer = String::new();
        let mut esc = false;

        while let Some(&curr) = self.chars.peek() {
            if curr == '"' || curr == '\n' { // end string if newline or closing quote
                break;
            }

            if esc {
                match char_maps::get_esc(curr) {
                    Some(escaped) => buffer.push(*escaped),
                    None => {
                        dispatch_error(ErrorType::UnknownEscapeSequence(curr), Some(self.position));
                        panic!(); // (not called) avoid incompatible arm type error
                    }
                }
                esc = false;
            } else if curr == '\\' { // escape character
                esc = true;
            } else {
                buffer.push(curr);
            }

            self.advance();
        }

        match self.chars.peek() {
            Some(&c) => if c != '"' { dispatch_error(ErrorType::UnclosedString, Some(self.position)) }
            None => dispatch_error(ErrorType::UnclosedString, Some(self.position))
        }

        self.advance();
        Token::new(TokenType::String, Some(buffer), start)
    }

    fn consume_single(&mut self) -> Token<'a> {
        let &c = match self.chars.peek() {
            Some(c) => c,
            None => {
                dispatch_error(ErrorType::GenericError("single consume"), None);
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        let token = match char_maps::get_single(c) {
            Some((token, wrap)) => {
                match wrap {
                    Some(opening) => {
                        if *opening {
                            self.bracket_count += 1;
                        } else if self.bracket_count > 0 {
                            self.bracket_count -= 1;
                        }
                    }
                    _ => {}
                };
                *token
            }
            None => {
                dispatch_error(ErrorType::UnknownChar(c), Some(self.position));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        let start = self.position;
        self.advance();

        Token::new(token, None, start)
    }

    fn consume_comment(&mut self) {
        while let Some(&curr) = self.chars.peek() {
            if curr == '\n' {
                self.advance_newline();
                break;
            }
            self.advance();
        }
    }

    fn consume_identifier(&mut self) -> Token<'a> {
        let start = self.position;
        let mut buffer = String::new();

        while let Some(&curr) = self.chars.peek() {
            if curr != '_' && !curr.is_ascii_digit() && !curr.is_ascii_alphabetic() {
                break;
            }

            buffer.push(curr);
            self.advance();
        }

        if buffer.len() < 1 {
            match self.chars.peek() {
                Some(&c) => dispatch_error(ErrorType::UnknownChar(c), Some(self.position)),
                None => { dispatch_error(ErrorType::GenericError("consume ident"), None) }
            }
        }

        match char_maps::get_token(&buffer) {
            Some(token) => Token::new(*token, None, start),
            None => Token::new(TokenType::Identifier, Some(buffer), start)
        }
    }

    fn consume_newline(&mut self) -> Option<Token<'a>> {
        if self.newline || self.bracket_count > 0 {
            return None;
        }

        let start = self.position;
        self.advance_newline();

        Some(Token::new(TokenType::Newline, None, start))
    }

    fn is_newline(c: char) -> bool {
        "\n;".contains(c)
    }

    fn is_operator(c: char) -> bool {
        "+-*/%<>=!".contains(c)
    }

    fn advance(&mut self) {
        self.position.advance(false);
        self.chars.next();
    }

    fn advance_newline(&mut self) {
        self.position.advance(true);
        self.chars.next();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.peek() {
            Some(&curr) => {
                if Self::is_newline(curr) {
                    return match self.consume_newline() {
                        Some(token) => {
                            self.newline = true;
                            Some(token)
                        }
                        None => {
                            self.advance_newline();
                            self.next()
                        }
                    };
                } else if curr.is_ascii_whitespace() {
                    self.advance();
                    self.next()
                } else if curr.is_ascii_digit() {
                    self.newline = false;
                    Some(self.consume_number())
                } else if curr == '#' {
                    self.newline = false;
                    self.consume_comment();
                    self.next()
                } else if char_maps::is_single(curr) {
                    self.newline = false;
                    Some(self.consume_single())
                } else if curr == '"' {
                    self.newline = false;
                    Some(self.consume_string())
                } else if Self::is_operator(curr) {
                    self.newline = false;
                    Some(self.consume_operator())
                } else {
                    self.newline = false;
                    Some(self.consume_identifier())
                }
            }
            None => None
        }
    }
}