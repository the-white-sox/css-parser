use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeSelector {
    WithoutOperator(String),
    WithOperator(String, AttributeSelectorOperator, String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeSelectorOperator {
    /// =
    Equals,
    /// ~=
    ListContains,
    /// ^=
    StartsWith,
    /// $=
    EndsWith,
    /// *=
    StringContains,
}
