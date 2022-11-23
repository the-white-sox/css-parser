use super::rule::*;
use super::*;

pub struct MediaRule {
    pub media_queries: Vec<MediaQuery>,
    pub rules: Vec<Rule>,
}

pub struct MediaQuery {}

impl Parsable for Vec<MediaQuery> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
