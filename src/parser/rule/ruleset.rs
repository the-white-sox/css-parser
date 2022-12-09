use super::*;

#[derive(Debug, PartialEq)]
pub struct Ruleset {}

impl Parsable for Ruleset {
    fn parse<I: Iterator<Item = char>>(_parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
