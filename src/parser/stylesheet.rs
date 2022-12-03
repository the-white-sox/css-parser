use super::{import::Import, rule::Rule, *};

#[derive(Debug, PartialEq)]
pub struct Stylesheet {
    pub imports: Vec<Import>,
    pub rules: Vec<Rule>,
}

impl FromStr for Stylesheet {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Parser::new(input.chars()).into_stylesheet()
    }
}

impl Parsable for Stylesheet {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let mut imports = Vec::new();

        while let Some(token_at) = parser.tokens.peek() {
            match &token_at.token {
                Token::AtKeyword(keyword) if keyword == "import" => {
                    imports.push(parser.parse()?);
                    parser.optional_whitespace();
                }
                _ => break,
            }
        }

        let rules = parser.parse()?;

        if let Some(token_at) = parser.tokens.next() {
            Err(ParsingError::wrong_token(token_at, "end of file"))?
        }

        Ok(Stylesheet { imports, rules })
    }
}

#[cfg(test)]
mod tests {

    use crate::parser::url::Url;

    use super::*;

    #[test]
    fn empty() {
        let mut parser = Parser::new("".chars());

        assert_eq!(
            Ok(Stylesheet {
                imports: vec![],
                rules: vec![]
            }),
            parser.parse()
        );

        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn import() {
        let mut parser = Parser::new("@import url(example.com);".chars());

        assert_eq!(
            Ok(Stylesheet {
                imports: vec![Import {
                    url: Url("example.com".to_owned()),
                    media_queries: vec![]
                }],
                rules: vec![]
            }),
            parser.parse()
        );

        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn imports() {
        let mut parser = Parser::new(
            "@import url(example.com/1);\n@import url(example.com/2);\n@import url(example.com/3);"
                .chars(),
        );
        assert_eq!(
            Ok(Stylesheet {
                imports: vec![
                    Import {
                        url: Url("example.com/1".to_owned()),
                        media_queries: vec![]
                    },
                    Import {
                        url: Url("example.com/2".to_owned()),
                        media_queries: vec![]
                    },
                    Import {
                        url: Url("example.com/3".to_owned()),
                        media_queries: vec![]
                    }
                ],
                rules: vec![]
            }),
            parser.parse()
        );

        assert_eq!(None, parser.tokens.next());
    }
}
