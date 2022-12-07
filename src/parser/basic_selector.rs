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
                        _ => Err(ParsingError::wrong_token(token_at, "Basic selector")),
                    },
                    None => Err(ParsingError::end_of_file("Basic Selector")),
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
    fn universalselector() {
        let mut parser = Parser::new("*".chars());
        assert_eq!(Ok(BasicSelector::Universal()), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn elementselector() {
        let mut parser = Parser::new("elementname".chars());
        assert_eq!(
            Ok(BasicSelector::Element("elementname".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn classselector() {
        let mut parser = Parser::new(".classname".chars());
        assert_eq!(
            Ok(BasicSelector::Class("classname".to_owned())),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn idselector() {
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
    fn attribute_selector_w_operator_1() {
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
    fn attribute_selector_w_operator_2() {
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
    fn attribute_selector_w_operator_3() {
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
    fn attribute_selector_w_operator_4() {
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
    fn attribute_selector_w_operator_5() {
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
}
