use super::{combinator::Combinator, *};

#[derive(Debug, PartialEq, Eq)]
pub enum RelativeSelector {
    WithCombinator(Combinator),
    WithoutCombinator(Selector),
}
