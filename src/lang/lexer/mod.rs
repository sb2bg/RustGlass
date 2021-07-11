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
                    dispatch_error(ErrorType::UnknownChar, Some(&[error.to_string().as_str()]), Some(PositionFragment::new(value, pos)));
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
                CharState::Newline => {
                    self.advance();
                    tokens.push(Token::new(TokenType::Newline, None, PositionFragment::new(self.source, self.position)));
                }
                CharState::Number => { self.consume_number(); }
                CharState::Operator => {}
                CharState::Quotation => {}
                CharState::SingleConsumable => {}
                CharState::Letter => {}
                CharState::Period => {}
                CharState::Underscore => {}
                CharState::Escape => {}
                CharState::Comment => {}
                CharState::Eof => {}
            }
        }

        return tokens;
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


        match char_maps::get(buffer) {
            Ok(operator) => return Token::new(operator, None, PositionFragment::new(self.source, start)),
            Err(value) => {
                dispatch_error(ErrorType::InvalidOperator, Some(&[value.as_str()]), Some(PositionFragment::new(self.source, self.position)));
                panic!(); // (not called) avoid incompatible arm type error
            }
        }
    }

    fn advance(&mut self) -> char {
        let &prev = &self.current;
        self.index += 1;
        self.current = if self.is_done() { '\0' } else { self.value[self.index] };
        self.position.advance(self.state == CharState::Newline);
        self.state = match state_from_char(self.current) {
            Ok(value) => value,
            Err(error) => {
                dispatch_error(ErrorType::UnknownChar, Some(&[error.to_string().as_str()]), Some(PositionFragment::new(self.source, self.position)));
                panic!(); // (not called) avoid incompatible arm type error
            }
        };


        return prev;
    }

    fn is_done(&self) -> bool {
        return self.index >= self.value.len() || self.current == '\0';
    }
}

