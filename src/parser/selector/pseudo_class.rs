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

const EXPECTED: &str = "focus, focus-within, focus-visible, hover, visited, default, active, target, root, checked, not(), or has()";

impl Parsable for PseudoClass {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        parser.expect(Token::Colon())?;

        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(pseudo_class_name) => match pseudo_class_name.as_str() {
                    "focus" => Ok(PseudoClass::Focus),
                    "focus-within" => Ok(PseudoClass::FocusWithin),
                    "focus-visible" => Ok(PseudoClass::FocusVisible),
                    "hover" => Ok(PseudoClass::Hover),
                    "visited" => Ok(PseudoClass::Visited),
                    "default" => Ok(PseudoClass::Default),
                    "active" => Ok(PseudoClass::Active),
                    "target" => Ok(PseudoClass::Target),
                    "root" => Ok(PseudoClass::Root),
                    "checked" => Ok(PseudoClass::Checked),
                    _ => Err(ParsingError::wrong_token(token_at, EXPECTED)),
                },
                Token::Function(pseudo_class_name) => match pseudo_class_name.as_str() {
                    "not" => Ok(PseudoClass::Not(parser.parse()?)),
                    "has" => Ok(PseudoClass::Has(parser.parse()?)),
                    _ => Err(ParsingError::wrong_token(token_at, EXPECTED)),
                },
                _ => Err(ParsingError::wrong_token(token_at, EXPECTED)),
            },
            None => Err(ParsingError::end_of_file(EXPECTED)),
        }
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

    #[test]
    fn missing_colon() {
        let mut parser = Parser::new("focus".chars());
        assert!(parser.parse::<PseudoClass>().is_err());
    }

    #[test]
    fn invalid_pseudo_class() {
        let mut parser = Parser::new(":invalid".chars());
        assert!(parser.parse::<PseudoClass>().is_err());
    }
}
