use super::*;
use crate::tokenizer::*;

impl Parsable for String {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::String(string) => Ok(string),
                _ => Err(ParsingError::wrong_token(token_at, "a string")),
            },

            None => Err(ParsingError::end_of_file("a string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut parser = Parser::new("'this is a string'".chars());
        let result = parser.parse::<String>().unwrap();
        assert_eq!(result, String::from("this is a string"));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn not_string() {
        let mut parser = Parser::new("not a string".chars());
        assert!(parser.parse::<String>().is_err());
    }

    #[test]
    fn nothing() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<String>().is_err());
    }
}
