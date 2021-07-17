pub mod fragment;

#[derive(Clone)]
pub struct Position {
    filename: String,
    absolute_index: i32,
    column: usize,
    row: usize,
}

impl Position {
    pub fn new(filename: String) -> Self {
        return Position { filename, absolute_index: 0, column: 1, row: 1 };
    }

    pub fn advance(&mut self, newline: bool) -> &Position {
        self.column = if newline { 1 } else { self.column + 1 };
        self.row = if newline { self.row + 1 } else { self.row };
        self.absolute_index += 1;

        return self;
    }

    pub fn to_string(&self) -> String {
        return format_args!("[{}(Ln:{} Col:{})]", self.filename, self.row, self.column).to_string();
    }
}