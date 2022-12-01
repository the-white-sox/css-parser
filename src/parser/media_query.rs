use super::*;
use crate::tokenizer::*;

mod media_feature;
mod media_type;

use media_feature::*;
use media_type::*;

#[derive(Debug, PartialEq)]
pub enum MediaQuery {
    MediaType(MediaType),
    MediaFeature(MediaFeature),
    Not(Box<MediaQuery>),
    And(Box<MediaQuery>, Box<MediaQuery>),
    Or(Box<MediaQuery>, Box<MediaQuery>),
}

impl Parsable for MediaQuery {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn media_type() {
        let mut parser = Parser::new("screen".chars());
        assert_eq!(Ok(MediaQuery::MediaType(MediaType::Screen)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn media_feature() {
        let mut parser = Parser::new("(color)".chars());
        assert_eq!(
            Ok(MediaQuery::MediaFeature(MediaFeature::Color)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn media_feature_without_parentheses() {
        let mut parser = Parser::new("color".chars());
        assert!(parser.parse::<MediaQuery>().is_err());
    }

    #[test]
    fn not() {
        let mut parser = Parser::new("not screen".chars());
        assert_eq!(
            Ok(MediaQuery::Not(Box::new(MediaQuery::MediaType(
                MediaType::Screen
            )))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn not_with_parentheses() {
        let mut parser = Parser::new("not (screen)".chars());
        assert_eq!(
            Ok(MediaQuery::Not(Box::new(MediaQuery::MediaType(
                MediaType::Screen
            )))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn and() {
        let mut parser = Parser::new("screen and (color)".chars());
        assert_eq!(
            Ok(MediaQuery::And(
                Box::new(MediaQuery::MediaType(MediaType::Screen)),
                Box::new(MediaQuery::MediaFeature(MediaFeature::Color))
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn or() {
        let mut parser = Parser::new("screen or (color)".chars());
        assert_eq!(
            Ok(MediaQuery::Or(
                Box::new(MediaQuery::MediaType(MediaType::Screen)),
                Box::new(MediaQuery::MediaFeature(MediaFeature::Color))
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn invalid_feature_with_and() {
        let mut parser = Parser::new("(color and screen)".chars());
        assert!(parser.parse::<MediaQuery>().is_err());
    }

    #[test]
    fn and_or() {
        // note that we do not have order of operations
        let mut parser = Parser::new("screen and (color) or (orientation: landscape)".chars());
        assert_eq!(
            Ok(MediaQuery::And(
                Box::new(MediaQuery::MediaType(MediaType::Screen)),
                Box::new(MediaQuery::Or(
                    Box::new(MediaQuery::MediaFeature(MediaFeature::Color)),
                    Box::new(MediaQuery::MediaFeature(MediaFeature::Orientation(
                        Orientation::Landscape
                    )))
                ))
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn parentheses() {
        let mut parser = Parser::new("(screen)".chars());
        assert_eq!(Ok(MediaQuery::MediaType(MediaType::Screen)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn parentheses_with_whitespace() {
        let mut parser = Parser::new("( screen )".chars());
        assert_eq!(Ok(MediaQuery::MediaType(MediaType::Screen)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn parentheses_with_whitespace_and_and() {
        let mut parser = Parser::new("( screen and (color) )".chars());
        assert_eq!(
            Ok(MediaQuery::And(
                Box::new(MediaQuery::MediaType(MediaType::Screen)),
                Box::new(MediaQuery::MediaFeature(MediaFeature::Color))
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn lots_of_parentheses() {
        let mut parser = Parser::new("((screen and (color)) or (((print))))".chars());
        assert_eq!(
            Ok(MediaQuery::Or(
                Box::new(MediaQuery::And(
                    Box::new(MediaQuery::MediaType(MediaType::Screen)),
                    Box::new(MediaQuery::MediaFeature(MediaFeature::Color))
                )),
                Box::new(MediaQuery::MediaType(MediaType::Print))
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }
}
