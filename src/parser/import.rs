use super::media_query::*;
use super::url::*;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Import {
    pub url: Url,
    pub media_queries: Vec<MediaQuery>,
}

impl Parsable for Import {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::AtKeyword(keyword) if keyword == "import" => {
                    parser.optional_whitespace();

                    let url: Url = parser.parse_url_or_string()?;

                    parser.optional_whitespace();

                    let media_queries: Vec<MediaQuery> = match parser.tokens.peek() {
                        Some(token_at) => match token_at.token {
                            Token::Identifier(_) | Token::OpenParenthesis() => parser.parse()?,
                            Token::Semicolon() => Vec::new(),
                            _ => Err(ParsingError::wrong_token(
                                token_at.clone(),
                                "a semicolon or media query",
                            ))?,
                        },
                        None => Err(ParsingError::end_of_file("a semicolon or media query"))?,
                    };

                    parser.optional_whitespace();

                    parser.expect(Token::Semicolon())?;

                    Ok(Import { url, media_queries })
                }
                _ => Err(ParsingError::wrong_token(token_at, "@import")),
            },

            None => Err(ParsingError::end_of_file("@import")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_url() {
        let mut parser = Parser::new("@import url(example.com);".chars());
        assert_eq!(
            Ok(Import {
                url: Url("example.com".to_owned()),
                media_queries: vec![]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn extra_whitespace() {
        let mut parser = Parser::new("@import    url(example.com)  ;".chars());

        assert_eq!(
            Ok(Import {
                url: Url("example.com".to_owned()),
                media_queries: vec![]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn string_url() {
        let mut parser = Parser::new("@import 'example.com';".chars());
        assert_eq!(
            Ok(Import {
                url: Url("example.com".to_owned()),
                media_queries: vec![]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn media_query() {
        let mut parser = Parser::new("@import url(example.com) screen;".chars());
        assert_eq!(
            Ok(Import {
                url: Url("example.com".to_owned()),
                media_queries: vec![MediaQuery::MediaType(MediaType::Screen)]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn complex_media_queries() {
        let mut parser = Parser::new(
            "@import url(example.com) screen and (orientation: landscape), (color);".chars(),
        );
        assert_eq!(
            Ok(Import {
                url: Url("example.com".to_owned()),
                media_queries: vec![
                    MediaQuery::And(
                        Box::new(MediaQuery::MediaType(MediaType::Screen)),
                        Box::new(MediaQuery::MediaFeature(MediaFeature::Orientation(
                            Orientation::Landscape
                        )))
                    ),
                    MediaQuery::MediaFeature(MediaFeature::Color)
                ]
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }
}
