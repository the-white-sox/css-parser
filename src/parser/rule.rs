use super::media_query::*;

pub enum Rule {
    Ruleset(Ruleset),
    MediaRule(MediaRule),
}

pub struct Ruleset {}
