use super::media_query::*;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Ruleset(Ruleset),
    MediaRule(MediaRule),
}

#[derive(Debug, PartialEq)]
pub struct Ruleset {}
