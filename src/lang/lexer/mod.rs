use std::thread::current;

use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::char_state::{CharState, state_from_char};
use crate::lang::lexer::position::fragment::PositionFragment;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;

mod token;
mod char_maps;
mod char_state;
pub mod position;

struct Lexer {
    filename: &'static str,
    value: Vec<char>,
    source: &'static str,
    index: usize,
    position: Position,
    current: char,
    state: CharState,
}

impl Lexer {
    pub fn new(filename: &'static str, value: &'static str) -> Lexer {
        let chars: Vec<char> = value.chars().collect();
        let first = if chars.len() <= 0 { '\0' } else { *chars.get(0).unwrap() };
        let pos = Position::new(filename);

        return Lexer {
            filename,
            source: value,
            value: chars,
            index: 0,
            current: first,
            position: pos,
            state: match state_from_char(first) {
                Ok(value) => value,
                Err(error) => {
                    dispatch_error(ErrorType::UnknownChar, Some(&[error.to_string()]), Some(PositionFragment::new(value, pos)));
                    CharState::Eof
                }
            },
        };
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_done() {
            match self.state {
                CharState::Whitespace => { self.advance(); }
                CharState::Newline => { tokens.push(self.consume_newline()); }
                CharState::Number => { tokens.push(self.consume_number()); }
                CharState::Operator => { tokens.push(self.consume_operator()); }
                CharState::Quotation => { tokens.push(self.consume_string()); }
                CharState::SingleConsumable => { tokens.push(self.consume_single()); }
                CharState::Comment => { self.consume_comment(); }
                _ => { tokens.push(self.consume_identifier()); }
            }
        }

        return tokens;
    }

    fn consume_newline(&mut self) -> Token {
        self.advance();
        return Token::new(TokenType::Newline, None, PositionFragment::new(self.source, self.position));
    }

    fn consume_number(&mut self) -> Token {
        let start = self.position.clone();
        let mut buffer = String::new();
        let mut dec = false;

        while !self.is_done() && self.state.matches(&[CharState::Number, CharState::Period]) {
            if self.state == CharState::Period {
                if dec {
                    dispatch_error(ErrorType::DoubleDecimal, None, Some(PositionFragment::new(self.source, self.position)));
                }
                dec = true;
            }

            buffer.push(self.current);
            self.advance();
        }

        if buffer.ends_with('.') {
            dispatch_error(ErrorType::DecimalEnding, None, Some(PositionFragment::new(self.source, self.position)));
        }

        return Token::new(TokenType::Number, Some(buffer), PositionFragment::new(self.source, start));
    }

    fn consume_operator(&mut self) -> Token {
        let start = self.position.clone();
        let mut buffer = String::new();

        while !self.is_done() && self.state == CharState::Operator
        {
            buffer.push(self.current);
            self.advance();
        }


        match char_maps::get_token(buffer) {
            Ok(operator) => return Token::new(operator, None, PositionFragment::new(self.source, start)),
            Err(value) => {
                dispatch_error(ErrorType::InvalidOperator, Some(&[value]), Some(PositionFragment::new(self.source, self.position)));
                panic!(); // (not called) avoid incompatible arm type error
            }
        }
    }

    // todo - test strings and all that fun stuff!
    fn consume_string(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();
        let mut buffer = String::new();
        let mut esc = false;

        while !self.is_done() && self.state != CharState::Quotation {
            if esc {
                match char_maps::get_esc(self.current) {
                    Ok(escaped) => buffer.push(escaped),
                    Err(unknown) => {
                        dispatch_error(ErrorType::UnknownEscapeSequence, Some(&[unknown.to_string()]), Some(PositionFragment::new(self.source, self.position)));
                        panic!(); // (not called) avoid incompatible arm type error
                    }
                }
                esc = false;
            } else if self.state == CharState::Escape {
                esc = true;
            } else {
                buffer.push(self.current);
            }

            self.advance();
        }

        if self.state != CharState::Quotation {
            dispatch_error(ErrorType::UnclosedString, None, Some(PositionFragment::new(self.source, self.position)));
        }

        self.advance();
        return Token::new(TokenType::Str, Some(buffer), PositionFragment::new(self.source, start));
    }

    fn consume_single(&mut self) -> Token {
        let start = self.position.clone();

        let token = match char_maps::get_token(String::from(self.current)) {
            Ok(token) => token,
            Err(single) => {
                dispatch_error(ErrorType::UnknownChar, Some(&[single.to_string()]), Some(PositionFragment::new(self.source, self.position)));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        return Token::new(token, None, PositionFragment::new(self.source, start));
    }

    fn consume_comment(&mut self) {
        while !self.is_done() && self.state != CharState::Newline {
            self.advance();
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let start = self.position.clone();
        let mut buffer = String::new();

        while !self.is_done() && self.state.matches(&[CharState::Letter, CharState::Number, CharState::Underscore])
        {
            buffer.push(self.current);
            self.advance();
        }

        return match char_maps::get_token(buffer) {
            Ok(token) => Token::new(token, None, PositionFragment::new(self.source, start)),
            Err(value) => Token::new(TokenType::Identifier, Some(value), PositionFragment::new(self.source, start))
        };
    }

    fn advance(&mut self) -> char {
        let &prev = &self.current;
        self.index += 1;
        self.current = if self.is_done() { '\0' } else { self.value[self.index] };
        self.position.advance(self.state == CharState::Newline);
        self.state = match state_from_char(self.current) {
            Ok(value) => value,
            Err(error) => {
                dispatch_error(ErrorType::UnknownChar, Some(&[error.to_string()]), Some(PositionFragment::new(self.source, self.position)));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };


        return prev;
    }

    fn is_done(&self) -> bool {
        return self.index >= self.value.len() || self.current == '\0';
    }
}

