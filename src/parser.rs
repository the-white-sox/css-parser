use std::fmt;

use crate::tokenizer::Tokenizer;

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
    for token in Tokenizer::new(input.chars()) {
        println!("{:?}", token);
    }
    Ok(())
}
