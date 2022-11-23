use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Url {
    pub url: String,
}

impl Parsable for Url {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::Url(url) => Ok(Url { url }),
                Token::Function(name) if name == "url" => {
                    let url: String = parser.parse()?;
                    parser.expect(Token::CloseParenthesis())?;
                    Ok(Url { url })
                }
                _ => Err(ParsingError::wrong_token(token_at, "url")),
            },

            None => Err(ParsingError::end_of_file("url")),
        }
    }
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn parse_url_or_string(&mut self) -> Result<Url, ParsingError> {
        match self.tokens.peek() {
            Some(token_at) => match token_at.token {
                Token::String(_) => Ok(Url { url: self.parse()? }),
                Token::Url(_) | Token::Function(_) => self.parse::<Url>(),
                _ => Err(ParsingError::wrong_token(
                    token_at.clone(),
                    "a url or a string",
                )),
            },

            None => Err(ParsingError::end_of_file("a url or a string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let mut parser = Parser::new("url(example.com)".chars());
        let result = parser.parse::<Url>().unwrap();
        assert_eq!(
            result,
            Url {
                url: "example.com".to_owned()
            }
        );
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn function() {
        let mut parser = Parser::new("url('example.com')".chars());
        let result = parser.parse::<Url>().unwrap();
        assert_eq!(
            result,
            Url {
                url: "example.com".to_owned()
            }
        );
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn not_url() {
        let mut parser = Parser::new("url".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn empty_url() {
        let mut parser = Parser::new("url()".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn not_string() {
        let mut parser = Parser::new("url(5)".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn empty_string() {
        let mut parser = Parser::new("url('')".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn not_closed() {
        let mut parser = Parser::new("url('example.com'".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn nothing() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<String>().is_err());
    }
}
