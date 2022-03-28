use crate::{dispatch_error, Lexer};
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;
use crate::lang::lexer::token::Token;
use crate::lang::lexer::token::token_type::TokenType;
use crate::lang::parser::node::bin_op_node::BinOpNode;
use crate::lang::parser::node::bool_node::BoolNode;
use crate::lang::parser::node::Node;
use crate::lang::parser::node::number_node::NumberNode;
use crate::lang::parser::node::string_node::StringNode;
use crate::lang::parser::node::unary_node::UnaryNode;
use crate::lang::parser::node::void_node::VoidNode;

pub mod node;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

macro_rules! token_matches {
    ($token:expr $(, $token_type:path)+) => {
        match $token.get_type() {
            $($token_type => true,)*
            _ => false,
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Box<dyn Node> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Box<dyn Node> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Box<dyn Node> {
        let mut left = self.parse_comparison();

        if self.at_end() {
            return left;
        }

        while token_matches!(self.peek(), TokenType::EqualEqual, TokenType::NotEqual) {
            let op = self.next_token().get_type();
            let right = self.parse_comparison();
            left = Box::new(BinOpNode::new(op, left, right));
        };

        left
    }

    fn parse_comparison(&mut self) -> Box<dyn Node> {
        let mut left = self.parse_term();

        if self.at_end() {
            left
        } else {
            while token_matches!(self.peek(), TokenType::EqualEqual, TokenType::NotEqual, TokenType::LessThan, TokenType::LessThanEqual, TokenType::GreaterThan, TokenType::GreaterThanEqual) {
                let op = self.next_token().get_type();
                let right = self.parse_term();
                left = Box::new(BinOpNode::new(op, left, right));
            };

            left
        }
    }

    fn parse_term(&mut self) -> Box<dyn Node> {
        let mut left = self.parse_factor();

        if self.at_end() {
            left
        } else {
            while token_matches!(self.peek(), TokenType::Plus, TokenType::Minus) {
                let op = self.next_token().get_type();
                let right = self.parse_factor();
                left = Box::new(BinOpNode::new(op, left, right));
            };

            left
        }
    }

    fn parse_factor(&mut self) -> Box<dyn Node> {
        let mut left = self.parse_unary();

        if self.at_end() {
            left
        } else {
            while token_matches!(self.peek(), TokenType::Times, TokenType::Divide, TokenType::Mod) {
                let op = self.next_token().get_type();
                let right = self.parse_unary();
                left = Box::new(BinOpNode::new(op, left, right));
            };

            left
        }
    }

    fn parse_unary(&mut self) -> Box<dyn Node> {
        if self.at_end() {
            self.parse_primary()
        } else {
            if token_matches!(self.peek(), TokenType::Plus, TokenType::Minus) {
                let op = self.next_token().get_type();
                Box::new(UnaryNode::new(op, self.parse_unary()))
            } else {
                self.parse_primary()
            }
        }
    }

    fn parse_primary(&mut self) -> Box<dyn Node> {
        let primary = self.next_token();

        match primary.get_type() {
            TokenType::True => Box::new(BoolNode::new(true)),
            TokenType::False => Box::new(BoolNode::new(false)),
            TokenType::Void => Box::new(VoidNode::new()),
            TokenType::String => Box::new(StringNode::new(primary.get_value())),
            TokenType::Lparen => {
                let expr = self.parse_expression();
                self.expect(TokenType::Rparen);
                expr
            }
            TokenType::Number => {
                let parsed = primary.get_value().parse();

                if let Ok(value) = parsed {
                    Box::new(NumberNode::new(value))
                } else {
                    dispatch_error!(ErrorType::GenericError("GlassLang failed to parse the lexed number. Please report this error to GitHub"), primary.take_pos());
                }
            }
            // todo: implement std functions... definitely streamline this in the future
            // TokenType::Print => Box::new(),
            // TokenType::Println => Box::new(),
            _ => todo!()
        }
    }

    // returns the token and doesn't advance the index
    // used for lookahead (cases such as variable declaration)
    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    // returns the token and advances the index if it's not the end of the file
    fn next_token(&mut self) -> &Token {
        if self.at_end() {
            dispatch_error!(ErrorType::ReachedEndOfFile);
        }

        let token = &self.tokens[self.index];
        self.index += 1;
        token
    }

    // if the end of the file has been reached (or will be reached when next_token is called)
    fn at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    // expect will advance the index if the token matches, otherwise it will throw
    fn expect(&mut self, token_type: TokenType) {
        let current = self.next_token();

        if current.get_type() != token_type {
            dispatch_error!(ErrorType::UnexpectedExpression(token_type, current.get_type()), current.take_pos());
        }
    }
}