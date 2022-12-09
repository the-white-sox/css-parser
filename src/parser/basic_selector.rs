use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq, Eq)]
pub enum BasicSelector {
    Universal(),
    Element(String),
    Class(String),
    Id(String),
    Attribute(String),
    AttributeWOperator(String, AttributeSelectorOperator, String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeSelectorOperator {
    /// =
    Equals,
    /// ~=
    ListContains,
    /// ^=
    StartsWith,
    /// $=
    EndsWith,
    /// *=
    StringContains,
}

impl Parsable for BasicSelector {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::Delimiter('*') => Ok(BasicSelector::Universal()),
                Token::Identifier(element_name) => Ok(BasicSelector::Element(element_name)),
                Token::Delimiter('.') => match parser.tokens.next() {
                    Some(token_at) => match token_at.token {
                        Token::Identifier(class_name) => Ok(BasicSelector::Class(class_name)),
                        _ => Err(ParsingError::wrong_token(token_at, "class name")),
                    },
                    None => Err(ParsingError::end_of_file("class name")),
                },
                Token::Hash(id_name, HashType::Id) => Ok(BasicSelector::Id(id_name)),
                Token::Hash(id_name, HashType::Unrestricted) => Err(ParsingError::WrongToken {
                    line: token_at.line,
                    column: token_at.column,
                    expected: "valid id".to_owned(),
                    found: id_name,
                }),
                Token::OpenSquareBracket() => match parser.tokens.next() {
                    Some(token_at) => match token_at.token {
                        Token::Identifier(attribute_name) => match parser.tokens.next() {
                            Some(token_at) => match token_at.token {
                                Token::CloseSquareBracket() => {
                                    Ok(BasicSelector::Attribute(attribute_name))
                                }
                                Token::Delimiter('=') => {
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(BasicSelector::AttributeWOperator(
                                        attribute_name,
                                        AttributeSelectorOperator::Equals,
                                        value,
                                    ))
                                }
                                Token::Delimiter('~') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(BasicSelector::AttributeWOperator(
                                        attribute_name,
                                        AttributeSelectorOperator::ListContains,
                                        value,
                                    ))
                                }
                                Token::Delimiter('^') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(BasicSelector::AttributeWOperator(
                                        attribute_name,
                                        AttributeSelectorOperator::StartsWith,
                                        value,
                                    ))
                                }
                                Token::Delimiter('$') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(BasicSelector::AttributeWOperator(
                                        attribute_name,
                                        AttributeSelectorOperator::EndsWith,
                                        value,
                                    ))
                                }
                                Token::Delimiter('*') => {
                                    parser.expect(Token::Delimiter('='))?;
                                    let value: String = parser.parse()?;
                                    parser.expect(Token::CloseSquareBracket())?;
                                    Ok(BasicSelector::AttributeWOperator(
                                        attribute_name,
                                        AttributeSelectorOperator::StringContains,
                                        value,
                                    ))
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
    fn universal_selector() {
        let mut parser = Parser::new("*".chars());
        assert_eq!(Ok(BasicSelector::Universal()), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn element_selector() {
        let mut parser = Parser::new("elementname".chars());
        assert_eq!(
            Ok(BasicSelector::Element("elementname".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn class_selector() {
        let mut parser = Parser::new(".classname".chars());
        assert_eq!(
            Ok(BasicSelector::Class("classname".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn id_selector() {
        let mut parser = Parser::new("#idname".chars());
        assert_eq!(Ok(BasicSelector::Id("idname".to_owned())), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_no_operators() {
        let mut parser = Parser::new("[attributename]".chars());
        assert_eq!(
            Ok(BasicSelector::Attribute("attributename".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_w_operator_equals() {
        let mut parser = Parser::new("[attributename='answer']".chars());
        assert_eq!(
            Ok(BasicSelector::AttributeWOperator(
                "attributename".to_owned(),
                AttributeSelectorOperator::Equals,
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_w_operator_list_contains() {
        let mut parser = Parser::new("[attributename~='answer']".chars());
        assert_eq!(
            Ok(BasicSelector::AttributeWOperator(
                "attributename".to_owned(),
                AttributeSelectorOperator::ListContains,
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_w_operator_starts_with() {
        let mut parser = Parser::new("[attributename^='answer']".chars());
        assert_eq!(
            Ok(BasicSelector::AttributeWOperator(
                "attributename".to_owned(),
                AttributeSelectorOperator::StartsWith,
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_w_operator_ends_with() {
        let mut parser = Parser::new("[attributename$='answer']".chars());
        assert_eq!(
            Ok(BasicSelector::AttributeWOperator(
                "attributename".to_owned(),
                AttributeSelectorOperator::EndsWith,
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn attribute_selector_w_operator_string_contains() {
        let mut parser = Parser::new("[attributename*='answer']".chars());
        assert_eq!(
            Ok(BasicSelector::AttributeWOperator(
                "attributename".to_owned(),
                AttributeSelectorOperator::StringContains,
                "answer".to_owned()
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn class_w_nums() {
        let mut parser = Parser::new(".123".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn bracket_w_nums() {
        let mut parser = Parser::new("[123]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn just_nums() {
        let mut parser = Parser::new("123".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn hash_nums() {
        let mut parser = Parser::new("#123".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_operator_numbers() {
        let mut parser = Parser::new("[12=3]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_value_is_number() {
        let mut parser = Parser::new("[attr=3]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_name_is_number() {
        let mut parser = Parser::new("[12='value']".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_value_missing_quotes() {
        let mut parser = Parser::new("[attr=value]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_operator_list_contains_numbers() {
        let mut parser = Parser::new("[12~=3]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_list_contains_value_is_number() {
        let mut parser = Parser::new("[attr~=3]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_name_list_contains_is_number() {
        let mut parser = Parser::new("[12~='value']".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }

    #[test]
    fn attribute_list_contains_value_missing_quotes() {
        let mut parser = Parser::new("[attr~=value]".chars());
        assert!(parser.parse::<BasicSelector>().is_err());
    }
}