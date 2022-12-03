use super::{media_query::*, *};

#[derive(Debug, PartialEq)]
pub enum Rule {
    Ruleset(Ruleset),
    MediaRule(MediaRule),
}

#[derive(Debug, PartialEq)]
pub struct Ruleset {}

#[derive(Debug, PartialEq)]
pub struct MediaRule {
    pub media_queries: Vec<MediaQuery>,
    pub rules: Vec<Rule>,
}

impl Parsable for Vec<Rule> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
