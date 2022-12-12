use super::*;

impl Parsable for Vec<Declaration> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let mut declarations = Vec::new();
        loop {
            parser.optional_whitespace();

            match parser.tokens.peek() {
                Some(TokenAt {
                    token: Token::Identifier(_),
                    ..
                }) => {
                    let dec: Declaration = parser.parse()?;
                    declarations.push(dec);
                }
                _ => break,
            }

            parser.optional_whitespace();

            match parser.tokens.peek() {
                Some(TokenAt {
                    token: Token::Semicolon(),
                    ..
                }) => {
                    parser.tokens.next();
                    continue;
                }
                _ => break,
            }
        }
        Ok(declarations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_dec() {
        let mut parser = Parser::new("background-color: red".chars());
        assert_eq!(
            Ok(vec![Declaration::BackgroundColor(Color::Red)]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn two_dec() {
        let mut parser = Parser::new("background-color: red; border-color: red;".chars());
        assert_eq!(
            Ok(vec![
                Declaration::BackgroundColor(Color::Red),
                Declaration::BorderColor(Color::Red)
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn two_dec_no_ending_semicolon() {
        let mut parser = Parser::new("background-color: red; border-color: red".chars());
        assert_eq!(
            Ok(vec![
                Declaration::BackgroundColor(Color::Red),
                Declaration::BorderColor(Color::Red)
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn missing_semicolon() {
        let mut parser = Parser::new("background-color: red border-color: red".chars());
        assert_eq!(
            Ok(vec![Declaration::BackgroundColor(Color::Red)]),
            parser.parse()
        );
        assert_ne!(None, parser.tokens.next());
    }
}
