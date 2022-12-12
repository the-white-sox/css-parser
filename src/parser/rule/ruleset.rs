use super::basic_selector::BasicSelector;
use super::declaration::Declaration;

use super::*;

#[derive(Debug, PartialEq)]
pub struct Ruleset {
    pub selectors: Vec<BasicSelector>,
    pub declarations: Vec<Declaration>,
}

impl Parsable for Ruleset {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let selectors = parser.parse()?;
        parser.expect(Token::Delimiter('{'))?;
        let declarations = parser.parse()?;
        parser.expect(Token::Delimiter('}'))?;
        Ok(Ruleset {
            selectors,
            declarations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::basic_selector::BasicSelector;
    use super::declaration::Declaration;
    use super::*;
    use crate::parser::color::Color;

    #[test]
    fn empty_ruleset() {
        let mut parser = Parser::new("* {}".chars());
        assert_eq!(
            Ok(Ruleset {
                selectors: vec![BasicSelector::Universal()],
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
                selectors: vec![BasicSelector::Universal()],
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
                    BasicSelector::Element("div".to_string()),
                    BasicSelector::Id("fab".to_string())
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
