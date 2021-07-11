use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;

pub struct PositionFragment {
    source: &'static str,
    pos: Position,
}

impl PositionFragment {
    pub fn new(source: &'static str, pos: Position) -> Self {
        return PositionFragment { source, pos };
    }

    pub fn get_pos(&self) -> Position {
        return self.pos;
    }

    pub fn to_string(&self) -> String {
        return format_args!("\n\n\t{}\n\t {}^\n{}", self.get_line((self.pos.row - 1) as usize), " ".repeat((self.pos.column - 1) as usize), self.pos.to_string()).to_string();
    }

    fn get_line(&self, line: usize) -> &str {
        let result = match self.source.lines().nth(line) {
            Some(line) => line,
            None => {
                dispatch_error(ErrorType::GenericError, Some(&[String::from("converting position to string")]), None);
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        return result;
    }
}