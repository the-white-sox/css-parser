use super::*;

/// Represents a percentage value
/// Grammar: `<percentage>`
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentage(pub f64);

impl Parsable for Percentage {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::Percentage(value) => Ok(Percentage(value)),
                _ => Err(ParsingError::wrong_token(token_at, "percentage")),
            },
            None => Err(ParsingError::end_of_file("percentage")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_no_value() {
        let mut parser = Parser::new("%".chars());
        assert!(parser.parse::<Percentage>().is_err());
    }

    #[test]
    fn zero() {
        let mut parser = Parser::new("0%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(0.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_int() {
        let mut parser = Parser::new("23%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(23.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_float() {
        let mut parser = Parser::new("62.3%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(62.3));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_int() {
        let mut parser = Parser::new("-87%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(-87.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_float() {
        let mut parser = Parser::new("-4.20%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(-4.20));
        assert!(parser.tokens.next().is_none());
    }
}
