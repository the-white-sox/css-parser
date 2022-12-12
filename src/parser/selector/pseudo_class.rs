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

impl Parsable for PseudoClass {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus() {
        let mut parser = Parser::new(":focus".chars());
        assert_eq!(Ok(PseudoClass::Focus), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn focus_within() {
        let mut parser = Parser::new(":focus-within".chars());
        assert_eq!(Ok(PseudoClass::FocusWithin), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn focus_visible() {
        let mut parser = Parser::new(":focus-visible".chars());
        assert_eq!(Ok(PseudoClass::FocusVisible), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hover() {
        let mut parser = Parser::new(":hover".chars());
        assert_eq!(Ok(PseudoClass::Hover), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn visited() {
        let mut parser = Parser::new(":visited".chars());
        assert_eq!(Ok(PseudoClass::Visited), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn default() {
        let mut parser = Parser::new(":default".chars());
        assert_eq!(Ok(PseudoClass::Default), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn active() {
        let mut parser = Parser::new(":active".chars());
        assert_eq!(Ok(PseudoClass::Active), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn target() {
        let mut parser = Parser::new(":target".chars());
        assert_eq!(Ok(PseudoClass::Target), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn root() {
        let mut parser = Parser::new(":root".chars());
        assert_eq!(Ok(PseudoClass::Root), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn checked() {
        let mut parser = Parser::new(":checked".chars());
        assert_eq!(Ok(PseudoClass::Checked), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }
}
