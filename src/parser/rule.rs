use super::{media_query::*, *};

pub mod media_rule;
pub mod ruleset;

use media_rule::MediaRule;
use ruleset::Ruleset;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Ruleset(Ruleset),
    MediaRule(MediaRule),
}

impl Parsable for Vec<Rule> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let mut rules = Vec::new();

        loop {
            parser.optional_whitespace();

            match parser.tokens.peek() {
                Some(token_at) => match &token_at.token {
                    Token::AtKeyword(keyword) if keyword == "media" => {
                        rules.push(Rule::MediaRule(parser.parse()?));
                    }
                    Token::Identifier(_)
                    | Token::Hash(_, _)
                    | Token::Delimiter('.' | ':' | '*')
                    | Token::OpenSquareBracket()
                    | Token::Colon() => {
                        rules.push(Rule::Ruleset(parser.parse()?));
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Ok(rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut parser = Parser::new("".chars());
        assert_eq!(Ok(Vec::<Rule>::new()), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn media_rule() {
        let mut parser = Parser::new("@media screen { }".chars());
        assert_eq!(
            Ok(vec![Rule::MediaRule(MediaRule {
                media_queries: vec![MediaQuery::MediaType(MediaType::Screen)],
                rules: vec![]
            })]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn multiple_media_rules() {
        let mut parser = Parser::new("@media screen { } @media print { }".chars());
        assert_eq!(
            Ok(vec![
                Rule::MediaRule(MediaRule {
                    media_queries: vec![MediaQuery::MediaType(MediaType::Screen)],
                    rules: vec![]
                }),
                Rule::MediaRule(MediaRule {
                    media_queries: vec![MediaQuery::MediaType(MediaType::Print)],
                    rules: vec![]
                })
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    // TODO: Add tests for rulesets
}
