use super::media_query::*;
use super::url::*;
use super::*;

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

                    let media_queries: Vec<MediaQuery> = parser.parse()?;

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
        let result: Import = parser.parse().unwrap();
        assert_eq!(result.url.url, "example.com".to_owned());
        assert_eq!(result.media_queries.len(), 0);
    }

    #[test]
    fn extra_whitespace() {
        let mut parser = Parser::new("@import    url(example.com)  ;".chars());
        let result: Import = parser.parse().unwrap();
        assert_eq!(result.url.url, "example.com".to_owned());
        assert_eq!(result.media_queries.len(), 0);
    }
}
