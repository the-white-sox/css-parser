use super::*;

#[derive(Debug, PartialEq)]
pub struct MediaRule {
    pub media_queries: Vec<MediaQuery>,
    pub rules: Vec<Rule>,
}

impl Parsable for MediaRule {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        parser.expect(Token::AtKeyword("media".to_owned()))?;
        parser.optional_whitespace();

        let media_queries: Vec<MediaQuery> = parser.parse()?;

        parser.optional_whitespace();
        parser.expect(Token::OpenCurlyBracket())?;
        parser.optional_whitespace();

        let rules: Vec<Rule> = parser.parse()?;

        parser.optional_whitespace();
        parser.expect(Token::CloseCurlyBracket())?;

        Ok(MediaRule {
            media_queries,
            rules,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_queries() {
        let mut parser = Parser::new("@media {} ".chars());
        assert!(parser.parse::<MediaRule>().is_err());
    }

    #[test]
    fn one_query() {
        let mut parser = Parser::new("@media (color) {}".chars());

        assert_eq!(
            Ok(MediaRule {
                media_queries: vec![MediaQuery::MediaFeature(MediaFeature::Color)],
                rules: vec![]
            }),
            parser.parse()
        );

        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn two_queries() {
        let mut parser = Parser::new("@media (color), not screen {}".chars());

        assert_eq!(
            Ok(MediaRule {
                media_queries: vec![
                    MediaQuery::MediaFeature(MediaFeature::Color),
                    MediaQuery::Not(Box::new(MediaQuery::MediaType(MediaType::Screen)))
                ],
                rules: vec![]
            }),
            parser.parse()
        );

        assert_eq!(None, parser.tokens.next());
    }
}
