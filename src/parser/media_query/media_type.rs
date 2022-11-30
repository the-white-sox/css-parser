use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    All,
    Print,
    Screen,
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(MediaType::All),
            "print" => Ok(MediaType::Print),
            "screen" => Ok(MediaType::Screen),
            _ => Err(()),
        }
    }
}

impl Parsable for MediaType {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(string) => match string.parse() {
                    Ok(media_type) => Ok(media_type),
                    Err(_) => Err(ParsingError::wrong_token(token_at, "all, print, or screen")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "all, print, or screen")),
            },

            None => Err(ParsingError::end_of_file("all, print, or screen")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let mut parser = Parser::new("all".chars());
        let result = parser.parse::<MediaType>().unwrap();
        assert_eq!(result, MediaType::All);
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn print() {
        let mut parser = Parser::new("print".chars());
        let result = parser.parse::<MediaType>().unwrap();
        assert_eq!(result, MediaType::Print);
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn screen() {
        let mut parser = Parser::new("screen".chars());
        let result = parser.parse::<MediaType>().unwrap();
        assert_eq!(result, MediaType::Screen);
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn not_media_type() {
        let mut parser = Parser::new("not a media type".chars());
        assert!(parser.parse::<MediaType>().is_err());
    }

    #[test]
    fn nothing() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<MediaType>().is_err());
    }
}
