use super::*;

#[derive(Debug, PartialEq)]
pub enum Length {}

impl Parsable for Length {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
