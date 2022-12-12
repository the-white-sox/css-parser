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
    }
}
