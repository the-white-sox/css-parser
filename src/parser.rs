use std::iter::Peekable;
use std::{fmt, str::FromStr};

use crate::tokenizer::{Token, TokenAt, Tokenizer};

mod length;
mod percentage;
mod string;
mod url;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    WrongToken {
        line: usize,
        column: usize,
        expected: String,
        found: String,
    },
    EndOfFile {
        expected: String,
    },
}

impl ParsingError {
    fn wrong_token(token_at: TokenAt, expected: &str) -> Self {
        Self::WrongToken {
            line: token_at.line,
            column: token_at.column,
            expected: expected.to_owned(),
            found: token_at.token.to_string(),
        }
    }

    fn end_of_file(expected: &str) -> Self {
        Self::EndOfFile {
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongToken {
                line,
                column,
                expected,
                found,
            } => {
                write!(
                    formatter,
                    "Error on line {} column {} expected {} but found {}.",
                    line + 1,
                    column + 1,
                    expected,
                    found
                )
            }
            Self::EndOfFile { expected } => {
                write!(
                    formatter,
                    "Error at end of file expected {} but found nothing.",
                    expected,
                )
            }
        }
    }
}

pub struct Parser<I: Iterator<Item = char>> {
    tokens: Peekable<Tokenizer<I>>,
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn new(input: I) -> Self {
        Self {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    fn parse<T: Parsable>(&mut self) -> Result<T, ParsingError> {
        T::parse(self)
    }

    /// expect the next token to match a given token
    fn expect(&mut self, expected: Token) -> Result<(), ParsingError> {
        match self.tokens.next() {
            Some(token_at) => {
                if token_at.token == expected {
                    Ok(())
                } else {
                    Err(ParsingError::wrong_token(token_at, &expected.to_string()))
                }
            }

            None => Err(ParsingError::end_of_file("a string")),
        }
    }

    /// consume whitespace token if there are any
    fn optional_whitespace(&mut self) {
        while let Some(TokenAt {
            token: Token::Whitespace(),
            ..
        }) = self.tokens.peek()
        {
            self.tokens.next();
        }
    }

    pub fn into_stylesheet(mut self) -> Result<Stylesheet, ParsingError> {
        self.parse()
    }
}

pub trait Parsable: Sized {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError>;
}

pub struct Stylesheet {}

impl Parsable for Stylesheet {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        #[allow(clippy::while_let_on_iterator)] // because we are borrowing parser
        while let Some(TokenAt {
            line,
            column,
            token,
        }) = parser.tokens.next()
        {
            match token {
                Token::BadComment() => {
                    return Err(ParsingError::WrongToken {
                        line,
                        column,
                        expected: "comment to end".to_owned(),
                        found: "comment that never ends".to_owned(),
                    })
                }
                Token::BadString() => {
                    return Err(ParsingError::WrongToken {
                        line,
                        column,
                        expected: "string to end".to_owned(),
                        found: "string that never ends".to_owned(),
                    })
                }
                _ => {}
            }
        }
        Ok(Stylesheet {})
    }
}

impl FromStr for Stylesheet {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Parser::new(input.chars()).into_stylesheet()
    }
}
