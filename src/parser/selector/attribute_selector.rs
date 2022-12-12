use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeSelector {
    /// [key]
    Exists(String),
    /// [key=value]
    Equals(String, String),
    /// [key~=value]
    ListContains(String, String),
    /// [key^=value]
    StartsWith(String, String),
    /// [key$=value]
    EndsWith(String, String),
    /// [key*=value]
    StringContains(String, String),
}

impl Parsable for AttributeSelector {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::OpenSquareBracket() => match parser.tokens.next() {
                    Some(token_at) => match token_at.token {
                        Token::Identifier(attribute_name) => match parser.tokens.next() {
                            Some(token_at) => match token_at.token {
                                Token::CloseSquareBracket() => {
                                    Ok(AttributeSelector::Exists(attribute_name))
                                }
                                Token::Delimiter('=') => {
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(AttributeSelector::Equals(attribute_name, value))
                                }
                                Token::Delimiter('~') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(AttributeSelector::ListContains(attribute_name, value))
                                }
                                Token::Delimiter('^') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(AttributeSelector::StartsWith(attribute_name, value))
                                }
                                Token::Delimiter('$') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(AttributeSelector::EndsWith(attribute_name, value))
                                }
                                Token::Delimiter('*') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(AttributeSelector::StringContains(attribute_name, value))
                                }
                                _ => Err(ParsingError::wrong_token(
                                    token_at,
                                    "closing square bracket or attribute operator",
                                )),
                            },
                            None => Err(ParsingError::end_of_file(
                                "closing square bracket or attribute operator",
                            )),
                        },
                        _ => Err(ParsingError::wrong_token(token_at, "attribute name")),
                    },
                    None => Err(ParsingError::end_of_file("attribute name")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "Basic selector")),
            },

            None => Err(ParsingError::end_of_file("Basic Selector")),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn attribute_selector_no_operators() {
        let mut parser = Parser::new("[attributename]".chars());
        assert_eq!(
            Ok(AttributeSelector::Exists("attributename".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_with_operator_equals() {
        let mut parser = Parser::new("[attributename='answer']".chars());
        assert_eq!(
            Ok(AttributeSelector::Equals(
                "attributename".to_owned(),
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_with_operator_list_contains() {
        let mut parser = Parser::new("[attributename~='answer']".chars());
        assert_eq!(
            Ok(AttributeSelector::ListContains(
                "attributename".to_owned(),
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_with_operator_starts_with() {
        let mut parser = Parser::new("[attributename^='answer']".chars());
        assert_eq!(
            Ok(AttributeSelector::StartsWith(
                "attributename".to_owned(),
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_with_operator_ends_with() {
        let mut parser = Parser::new("[attributename$='answer']".chars());
        assert_eq!(
            Ok(AttributeSelector::EndsWith(
                "attributename".to_owned(),
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_with_operator_string_contains() {
        let mut parser = Parser::new("[attributename*='answer']".chars());
        assert_eq!(
            Ok(AttributeSelector::StringContains(
                "attributename".to_owned(),
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn empty_input() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn empty_attribute_selector() {
        let mut parser = Parser::new("[]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn bracket_w_nums() {
        let mut parser = Parser::new("[123]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_operator_numbers() {
        let mut parser = Parser::new("[12=3]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_value_is_number() {
        let mut parser = Parser::new("[attr=3]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_name_is_number() {
        let mut parser = Parser::new("[12='value']".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_value_missing_quotes() {
        let mut parser = Parser::new("[attr=value]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_operator_list_contains_numbers() {
        let mut parser = Parser::new("[12~=3]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_list_contains_value_is_number() {
        let mut parser = Parser::new("[attr~=3]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_name_list_contains_is_number() {
        let mut parser = Parser::new("[12~='value']".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }

    #[test]
    fn attribute_list_contains_value_missing_quotes() {
        let mut parser = Parser::new("[attr~=value]".chars());
        assert!(parser.parse::<AttributeSelector>().is_err());
    }
}
