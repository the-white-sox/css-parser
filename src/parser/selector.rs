pub mod attribute_selector;
pub mod combinator;
pub mod pseudo_class;
pub mod relative_selector;

use attribute_selector::AttributeSelector;
use combinator::Combinator;
use pseudo_class::PseudoClass;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Selector {
    element: Option<String>,
    restrictions: Vec<SelectorRestriction>,
    combinator: Option<Box<Combinator>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectorRestriction {
    Id(String),
    Class(String),
    Attribute(AttributeSelector),
    PseudoClass(PseudoClass),
}
