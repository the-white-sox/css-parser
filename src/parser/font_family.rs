use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FontFamily(pub Vec<String>);

impl Parsable for FontFamily {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let mut font_families = Vec::new();

        loop {
            match parser.tokens.next() {
                Some(token_at) => match token_at.token {
                    Token::Identifier(mut name) => {
                        loop {
                            parser.optional_whitespace();
                            match parser.tokens.peek() {
                                Some(token_at) => match &token_at.token {
                                    Token::Identifier(part) => {
                                        name.push(' ');
                                        name.push_str(&part);
                                        parser.tokens.next();
                                    }
                                    _ => break,
                                },
                                None => break,
                            }
                        }

                        font_families.push(name);
                    }
                    Token::String(name) => {
                        font_families.push(name);
                    }
                    _ => {
                        return Err(ParsingError::wrong_token(token_at, "identifier or string"));
                    }
                },
                None => return Err(ParsingError::end_of_file("identifier or string")),
            }
            parser.optional_whitespace();
            match parser.tokens.peek() {
                Some(TokenAt {
                    token: Token::Comma(),
                    ..
                }) => {
                    parser.tokens.next();
                }
                _ => break,
            }
            parser.optional_whitespace();
        }

        Ok(Self(font_families))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_word() {
        let mut parser = Parser::new("Arial".chars());
        assert_eq!(Ok(FontFamily(vec!["Arial".to_owned()])), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn quotes() {
        let mut parser = Parser::new("\"Times New Roman\"".chars());
        assert_eq!(
            Ok(FontFamily(vec!["Times New Roman".to_owned()])),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn no_quotes() {
        let mut parser = Parser::new("Times New Roman".chars());
        assert_eq!(
            Ok(FontFamily(vec!["Times New Roman".to_owned()])),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn commas() {
        let mut parser = Parser::new("Arial, \"Times New Roman\", serif".chars());
        assert_eq!(
            Ok(FontFamily(vec![
                "Arial".to_owned(),
                "Times New Roman".to_owned(),
                "serif".to_owned()
            ])),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    // some failing tests taken from https://developer.mozilla.org/en-US/docs/Web/CSS/font-family#valid_family_names

    #[test]
    fn spaces() {
        let mut parser = Parser::new("Goudy Bookletter 1911, sans-serif".chars());
        assert_eq!(
            Ok(FontFamily(vec!["Goudy Bookletter".to_owned()])),
            parser.parse()
        );
        assert_ne!(None, parser.tokens.next());
    }

    #[test]
    fn slash() {
        let mut parser = Parser::new("Red/Black, sans-serif".chars());
        assert_eq!(Ok(FontFamily(vec!["Red".to_owned()])), parser.parse());
        assert_ne!(None, parser.tokens.next());
    }

    #[test]
    fn missing_comma() {
        let mut parser = Parser::new("\"Lucida\" Grande, sans-serif".chars());
        assert_eq!(Ok(FontFamily(vec!["Lucida".to_owned()])), parser.parse());
        assert_ne!(None, parser.tokens.next());
    }
}
