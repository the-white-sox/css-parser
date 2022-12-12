use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Combinator {
    /// whitespace
    Desendants(Selector),
    /// >
    DirectChildren(Selector),
    /// ~
    SiblingSelect(Selector),
    /// +
    SecondElementMatch(Selector),
}
