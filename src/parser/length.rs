use super::*;

pub enum Distance {}

impl Parsable for Distance {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
