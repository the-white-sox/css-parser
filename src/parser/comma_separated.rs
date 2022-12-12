use super::*;

/// implement this trait for a type that can be in a comma separated non-empty list with no trailing comma
pub trait CommaSeparated: Parsable {}

impl<T: CommaSeparated> Parsable for Vec<T> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let mut list = Vec::new();

        loop {
            list.push(parser.parse()?);

            parser.optional_whitespace();

            match parser.tokens.peek() {
                Some(TokenAt {
                    token: Token::Comma(),
                    ..
                }) => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    continue;
                }

                _ => break,
            }
        }

        Ok(list)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Test;

    impl Parsable for Test {
        fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
            parser.expect(Token::Identifier("test".to_string()))?;
            Ok(Test)
        }
    }

    impl CommaSeparated for Test {}

    #[test]
    fn empty() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<Vec<Test>>().is_err());
    }

    #[test]
    fn single() {
        let mut parser = Parser::new("test".chars());
        assert_eq!(Ok(vec![Test]), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn multiple() {
        let mut parser = Parser::new("test, test, test".chars());
        assert_eq!(Ok(vec![Test, Test, Test]), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn no_whitespace() {
        let mut parser = Parser::new("test,test,test".chars());
        assert_eq!(Ok(vec![Test, Test, Test]), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn extra_whitespace() {
        let mut parser = Parser::new("test   ,   test   ,   test   ".chars());
        assert_eq!(Ok(vec![Test, Test, Test]), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn trailing_comma() {
        let mut parser = Parser::new("test, test, test,".chars());
        assert!(parser.parse::<Vec<Test>>().is_err());
    }

    #[test]
    fn missing_comma() {
        let mut parser = Parser::new("test, test test".chars());
        assert_eq!(Ok(vec![Test, Test]), parser.parse());
        assert_ne!(None, parser.tokens.next());
    }
}
