use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum RelativeSelector {
    WithCombinator(Combinator),
    WithoutCombinator(Selector),
}
