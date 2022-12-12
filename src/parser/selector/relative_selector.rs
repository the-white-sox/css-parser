use super::{combinator::Combinator, *};

#[derive(Debug, PartialEq, Eq)]
pub enum RelativeSelector {
    WithCombinator(Combinator),
    WithoutCombinator(Selector),
}

impl Parsable for RelativeSelector {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}
