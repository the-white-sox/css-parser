use super::*;

pub struct Stylesheet {}

impl Parsable for Stylesheet {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        #[allow(clippy::while_let_on_iterator)] // because we are borrowing parser
        while let Some(TokenAt {
            line,
            column,
            token,
        }) = parser.tokens.next()
        {
            match token {
                Token::BadComment() => {
                    return Err(ParsingError::WrongToken {
                        line,
                        column,
                        expected: "comment to end".to_owned(),
                        found: "comment that never ends".to_owned(),
                    })
                }
                Token::BadString() => {
                    return Err(ParsingError::WrongToken {
                        line,
                        column,
                        expected: "string to end".to_owned(),
                        found: "string that never ends".to_owned(),
                    })
                }
                _ => {}
            }
        }
        Ok(Stylesheet {})
    }
}

impl FromStr for Stylesheet {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Parser::new(input.chars()).into_stylesheet()
    }
}
