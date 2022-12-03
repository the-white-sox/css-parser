use super::{media_query::*, *};

pub mod media_rule;
pub mod ruleset;

use media_rule::MediaRule;
use ruleset::Ruleset;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Ruleset(Ruleset),
    MediaRule(MediaRule),
}

impl Parsable for Vec<Rule> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
