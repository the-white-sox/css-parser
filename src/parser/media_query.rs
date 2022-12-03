use super::*;
use crate::tokenizer::*;

mod media_feature;
mod media_type;
mod vec;

pub use media_feature::*;
pub use media_type::*;
pub use vec::*;

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
        let first = match parser.tokens.peek() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(name) if name == "not" => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    Ok(MediaQuery::Not(Box::new(parser.parse()?)))
                }

                Token::Identifier(_) => Ok(MediaQuery::MediaType(parser.parse()?)),

                Token::OpenParenthesis() => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    let inner = match parser.tokens.peek() {
                        Some(TokenAt {
                            token: Token::Identifier(name),
                            ..
                        }) if name.parse::<MediaType>().is_err() && name != "not" => {
                            Ok(MediaQuery::MediaFeature(parser.parse()?))
                        }
                        Some(_) => parser.parse(),
                        None => Err(ParsingError::end_of_file("media query")),
                    }?;
                    parser.optional_whitespace();
                    parser.expect(Token::CloseParenthesis())?;
                    Ok(inner)
                }

                _ => Err(ParsingError::wrong_token(
                    token_at.clone(),
                    "a media type or opening parenthesis",
                )),
            },

            None => Err(ParsingError::end_of_file("media query")),
        }?;

        parser.optional_whitespace();

        match parser.tokens.peek() {
            Some(TokenAt {
                token: Token::Identifier(name),
                ..
            }) if name == "and" => {
                parser.tokens.next();
                parser.optional_whitespace();
                Ok(MediaQuery::And(Box::new(first), Box::new(parser.parse()?)))
            }

            Some(TokenAt {
                token: Token::Identifier(name),
                ..
            }) if name == "or" => {
                parser.tokens.next();
                parser.optional_whitespace();
                Ok(MediaQuery::Or(Box::new(first), Box::new(parser.parse()?)))
            }

            _ => Ok(first),
        }
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
