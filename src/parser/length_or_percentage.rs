use super::{length::Length, percentage::Percentage, side::CanStart, *};

/// Wraps a length or percentage together
/// Grammar: `<length-or-percentage>
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl Parsable for LengthOrPercentage {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.peek() {
            Some(token_at) => match token_at.token {
                // parse length
                Token::Number(_) | Token::Dimension(_, _) => {
                    Ok(LengthOrPercentage::Length(parser.parse()?))
                }

                // parse percentage
                Token::Percentage(_) => Ok(LengthOrPercentage::Percentage(parser.parse()?)),

                // neither
                _ => Err(ParsingError::wrong_token(
                    token_at.clone(),
                    "length or percentage",
                )),
            },
            None => Err(ParsingError::end_of_file("length or percentage")),
        }
    }
}

impl CanStart for LengthOrPercentage {
    fn can_start(token: &Token) -> bool {
        matches!(
            token,
            Token::Number(_) | Token::Dimension(_, _) | Token::Percentage(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{length::LengthUnit, *};

    fn parse_length_or_percentage(input: &str) -> Result<LengthOrPercentage, ParsingError> {
        let mut parser = Parser::new(input.chars());
        return parser.parse::<LengthOrPercentage>();
    }

    #[test]
    fn nothing() {
        assert!(parse_length_or_percentage("").is_err());
    }

    #[test]
    fn percentage() {
        let mut parser = Parser::new("89%".chars());
        let result = parser.parse::<LengthOrPercentage>();

        assert_eq!(result, Ok(LengthOrPercentage::Percentage(Percentage(89.0))));
    }

    #[test]
    fn length() {
        let mut parser = Parser::new("35px".chars());
        let result = parser.parse::<LengthOrPercentage>();

        assert_eq!(
            result,
            Ok(LengthOrPercentage::Length(Length::Length(
                35.0,
                LengthUnit::Pixels
            )))
        );
    }
}
