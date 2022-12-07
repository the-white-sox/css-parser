use super::*;

// This trait signifies that a type can be parsed from an identifier using FromStr.
pub trait FromIdentifier: FromStr {
    // The expected identifier.
    const EXPECTED: &'static str;
}

impl<T: FromIdentifier> Parsable for T {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(string) => match string.parse() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(ParsingError::wrong_token(token_at, T::EXPECTED)),
                },
                _ => Err(ParsingError::wrong_token(token_at, T::EXPECTED)),
            },

            None => Err(ParsingError::end_of_file(T::EXPECTED)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// this struct is used to test the FromIdentifier trait
    struct MockFromStr(String);

    impl FromStr for MockFromStr {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "error" => Err(()),
                _ => Ok(MockFromStr(s.to_owned())),
            }
        }
    }

    // implement the FromIdentifier trait for MockFromStr
    impl FromIdentifier for MockFromStr {
        const EXPECTED: &'static str = "mock expected value";
    }

    #[test]
    fn identifier() {
        let mut parser = Parser::new("mock_input".chars());
        let result = parser.parse::<MockFromStr>().unwrap();
        assert_eq!(result.0, "mock_input");
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn identifier_error() {
        let mut parser = Parser::new("error".chars());
        assert!(parser.parse::<MockFromStr>().is_err());
    }

    #[test]
    fn not_identifier() {
        let mut parser = Parser::new("3 is not an identifier".chars());
        assert!(parser.parse::<MockFromStr>().is_err());
    }

    #[test]
    fn nothing() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<MockFromStr>().is_err());
    }
}
