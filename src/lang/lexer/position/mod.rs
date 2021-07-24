use crate::errorsystem::dispatch_error;
use crate::errorsystem::error_type::ErrorType;

#[derive(Copy, Clone)]
pub struct Position<'a> {
    source: &'a str,
    filename: &'a str,
    absolute_index: i32,
    column: usize,
    row: usize,
}

impl<'a> Position<'a> {
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        Position { filename, source, absolute_index: 0, column: 1, row: 1 }
    }

    pub fn advance(&mut self, newline: bool) -> &Position {
        self.column = if newline { 1 } else { self.column + 1 };
        self.row = if newline { self.row + 1 } else { self.row };
        self.absolute_index += 1;

        self
    }

    fn get_line(&self, line: usize) -> &str {
        let result = match self.source.lines().nth(line) {
            Some(line) => line,
            None => {
                dispatch_error(ErrorType::GenericError("converting position to string"), None);
                panic!(); // (not called) avoid incompatible arm type error
            }
        };

        result
    }
}

impl ToString for Position<'_> {
    fn to_string(&self) -> String {
        let line = self.row - 1;
        let col = self.column - 1;
        format!("\n\n\t\t{}\n\t\t{}^\n\t[{}(Ln:{} Col:{})]", self.get_line(line), if col > 0 { " ".repeat(col - 1) } else { String::new() }, self.filename, self.row, self.column)
    }
}