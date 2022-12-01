use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Url(String);

impl Parsable for Url {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::Url(url) => Ok(Url(url)),
                Token::Function(name) if name == "url" => {
                    parser.optional_whitespace();
                    let url: String = parser.parse()?;
                    parser.optional_whitespace();
                    parser.expect(Token::CloseParenthesis())?;
                    Ok(Url(url))
                }
                _ => Err(ParsingError::wrong_token(token_at, "url")),
            },

            None => Err(ParsingError::end_of_file("url")),
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
        assert_eq!(result, Url("example.com".to_owned()));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn function() {
        let mut parser = Parser::new("url('example.com')".chars());
        let result = parser.parse::<Url>().unwrap();
        assert_eq!(result, Url("example.com".to_owned()));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn function_with_whitespace() {
        let mut parser = Parser::new("url(   'example.com'   )".chars());
        let result = parser.parse::<Url>().unwrap();
        assert_eq!(result, Url("example.com".to_owned()));
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
