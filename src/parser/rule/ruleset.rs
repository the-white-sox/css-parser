use super::declaration::Declaration;
use super::selector::Selector;

use super::*;

#[derive(Debug, PartialEq)]
pub struct Ruleset {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Parsable for Ruleset {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let selectors = parser.parse()?;
        parser.expect(Token::OpenCurlyBracket())?;
        let declarations = parser.parse()?;
        parser.expect(Token::CloseCurlyBracket())?;
        Ok(Ruleset {
            selectors,
            declarations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::selector::SelectorRestriction;
    use super::*;
    use crate::parser::color::Color;

    const UNIVERSAL_SELECTOR: Selector = Selector {
        element: None,
        restrictions: vec![],
        combinator: None,
    };

    #[test]
    fn empty_ruleset() {
        let mut parser = Parser::new("* {}".chars());
        assert_eq!(
            Ok(Ruleset {
                selectors: vec![UNIVERSAL_SELECTOR],
                declarations: vec![]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn multiple_declarations() {
        let mut parser = Parser::new("* {background-color: blue; opacity: 0.7}".chars());
        assert_eq!(
            Ok(Ruleset {
                selectors: vec![UNIVERSAL_SELECTOR],
                declarations: vec![
                    Declaration::BackgroundColor(Color::Blue),
                    Declaration::Opacity(0.7)
                ]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn multiple_selectors() {
        let mut parser = Parser::new("div, #fab {background-color: blue}".chars());
        assert_eq!(
            Ok(Ruleset {
                selectors: vec![
                    Selector {
                        element: Some("div".to_owned()),
                        restrictions: vec![],
                        combinator: None
                    },
                    Selector {
                        element: None,
                        restrictions: vec![SelectorRestriction::Id("fab".to_owned())],
                        combinator: None
                    }
                ],
                declarations: vec![Declaration::BackgroundColor(Color::Blue)]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn no_selector() {
        let mut parser = Parser::new("{background-color: blue}".chars());
        assert!(parser.parse::<Ruleset>().is_err());
    }

    #[test]
    fn no_opening_brace() {
        let mut parser = Parser::new("* background-color: blue}".chars());
        assert!(parser.parse::<Ruleset>().is_err());
    }

    #[test]
    fn no_closing_brace() {
        let mut parser = Parser::new("* {background-color: blue".chars());
        assert!(parser.parse::<Ruleset>().is_err());
    }
}
