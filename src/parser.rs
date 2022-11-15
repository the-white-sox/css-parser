use std::fmt;

use crate::tokenizer::tokenize;

pub struct ParsingError {
    line: usize,
    column: usize,
    expected: String,
    found: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Error on line {} column {} expected {} but found {}.",
            self.line, self.column, self.expected, self.found
        )
    }
}

/// Parses a iterator of characters as css
pub fn parse(input: &str) -> Result<(), ParsingError> {
    tokenize(input);
    Ok(())
}
