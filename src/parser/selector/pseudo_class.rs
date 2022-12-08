use super::{relative_selector::RelativeSelector, *};

#[derive(Debug, PartialEq, Eq)]
pub enum PseudoClass {
    Focus,
    FocusWithin,
    FocusVisible,
    Hover,
    Visited,
    Default,
    Active,
    Target,
    Root,
    Checked,
    Not(RelativeSelector),
    Has(RelativeSelector),
}
