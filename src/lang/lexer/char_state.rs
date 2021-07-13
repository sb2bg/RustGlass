#[derive(PartialEq)]
pub enum CharState {
    Whitespace,
    Newline,
    Number,
    Operator,
    Quotation,
    SingleConsumable,
    Letter,
    Period,
    Underscore,
    Escape,
    Comment,
    Eof,
}

impl CharState {
    pub fn matches(&self, other: &[CharState]) -> bool {
        return other.contains(self);
    }
}

pub fn state_from_char(current: char) -> Result<CharState, char> {
    if current.is_ascii_digit() {
        return Ok(CharState::Number);
    } else if "\n;".contains(current) {
        return Ok(CharState::Newline);
    } else if current.is_whitespace() {
        return Ok(CharState::Whitespace);
    } else if current.is_ascii_alphabetic() {
        return Ok(CharState::Letter);
    } else if "+-*/%<>=!".contains(current) {
        return Ok(CharState::Operator);
    } else if "/,.()[]{}:".contains(current) {
        return Ok(CharState::SingleConsumable);
    } else if current == '"' {
        return Ok(CharState::Quotation);
    } else if current == '\\' {
        return Ok(CharState::Escape);
    } else if current == '.' {
        return Ok(CharState::Period);
    } else if current == '#' {
        return Ok(CharState::Comment);
    } else if current == '_' {
        return Ok(CharState::Underscore);
    } else if current == '\0' {
        return Ok(CharState::Eof);
    }

    return Err(current);
}