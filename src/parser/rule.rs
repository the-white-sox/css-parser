use super::media_query::*;

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
