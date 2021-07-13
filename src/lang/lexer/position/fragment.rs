use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;
use crate::lang::lexer::position::Position;

pub struct PositionFragment {
    source: String,
    pos: Position,
}

impl PositionFragment {
    pub fn new(source: String, pos: Position) -> Self {
        return PositionFragment { source, pos };
    }

    pub fn get_pos(&self) -> Position {
        return self.pos.clone();
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

impl ToString for PositionFragment {
    fn to_string(&self) -> String {
        let line = (self.pos.row - 1) as usize;
        let col = (self.pos.column - 1) as usize;
        return format_args!("\n\n\t\t{}\n\t\t{}^\n\t{}", self.get_line(line), if col > 0 { " ".repeat(col) } else { String::new() }, self.pos.to_string()).to_string();
    }
}