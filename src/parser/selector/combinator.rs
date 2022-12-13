use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Combinator {
    /// whitespace
    Descendant(Selector),
    /// >
    Child(Selector),
    /// ~
    GeneralSibling(Selector),
    /// +
    AdjacentSibling(Selector),
}

impl Parsable for Combinator {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.peek() {
            Some(token_at) => match &token_at.token {
                Token::Delimiter('>') => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    Ok(Combinator::Child(parser.parse()?))
                }
                Token::Delimiter('~') => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    Ok(Combinator::GeneralSibling(parser.parse()?))
                }
                Token::Delimiter('+') => {
                    parser.tokens.next();
                    parser.optional_whitespace();
                    Ok(Combinator::AdjacentSibling(parser.parse()?))
                }
                _ => Ok(Combinator::Descendant(parser.parse()?)),
            },
            None => Err(ParsingError::end_of_file("combinator")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNIVERSAL_SELECTOR: Selector = Selector {
        element: None,
        restrictions: vec![],
        combinator: None,
    };

    #[test]
    fn descendant() {
        let mut parser = Parser::new("*".chars());
        assert_eq!(
            Ok(Combinator::Descendant(UNIVERSAL_SELECTOR)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn child() {
        let mut parser = Parser::new("> *".chars());
        assert_eq!(Ok(Combinator::Child(UNIVERSAL_SELECTOR)), parser.parse());
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn general_sibling() {
        let mut parser = Parser::new("~ *".chars());
        assert_eq!(
            Ok(Combinator::GeneralSibling(UNIVERSAL_SELECTOR)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn adjacent_sibling() {
        let mut parser = Parser::new("+ *".chars());
        assert_eq!(
            Ok(Combinator::AdjacentSibling(UNIVERSAL_SELECTOR)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }
}
