use std::fmt;

use crate::tokenizer::{Token, TokenAt, Tokenizer};

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
    for TokenAt {
        line,
        column,
        token,
    } in Tokenizer::new(input.chars())
    {
        match token {
            Token::BadComment() => {
                return Err(ParsingError {
                    line,
                    column,
                    expected: "comment to end".to_owned(),
                    found: "comment that never ends".to_owned(),
                })
            }
            Token::BadString() => {
                return Err(ParsingError {
                    line,
                    column,
                    expected: "string to end".to_owned(),
                    found: "string that never ends".to_owned(),
                })
            }
            _ => {}
        }
    }
    Ok(())
}
