pub mod attribute_selector;
pub mod combinator;
pub mod pseudo_class;
pub mod relative_selector;

use attribute_selector::AttributeSelector;
use combinator::Combinator;
use pseudo_class::PseudoClass;

use crate::tokenizer::HashType;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Selector {
    element: Option<String>,
    restrictions: Vec<SelectorRestriction>,
    combinator: Option<Box<Combinator>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectorRestriction {
    Id(String),
    Class(String),
    Attribute(AttributeSelector),
    PseudoClass(PseudoClass),
}

impl Parsable for Selector {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let element = match parser.tokens.peek() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(element_name) => {
                    let element_name = element_name.clone();
                    parser.tokens.next();
                    Some(element_name)
                }
                Token::Delimiter('*') => {
                    parser.tokens.next();
                    None
                }
                Token::Hash(_, _)
                | Token::Delimiter('.')
                | Token::OpenSquareBracket()
                | Token::Colon() => None,
                _ => {
                    return Err(ParsingError::wrong_token(
                        token_at.clone(),
                        "*, element, id, class, attribute, or pseudo-class",
                    ))
                }
            },
            None => return Err(ParsingError::end_of_file("selector")),
        };

        let mut restrictions = Vec::new();

        loop {
            const EXPECTED: &str = "id, class, attribute, or pseudo-class";

            match parser.tokens.peek() {
                Some(token_at) => match &token_at.token {
                    Token::Hash(id_name, HashType::Id) => {
                        restrictions.push(SelectorRestriction::Id(id_name.clone()));
                        parser.tokens.next();
                    }
                    Token::Hash(id_name, HashType::Unrestricted) => {
                        return Err(ParsingError::WrongToken {
                            line: token_at.line,
                            column: token_at.column,
                            expected: "valid id".to_owned(),
                            found: id_name.clone(),
                        });
                    }
                    Token::Delimiter('.') => {
                        parser.tokens.next();
                        let Some(token_at) = parser.tokens.next() else {
                            return Err(ParsingError::end_of_file("class name"));
                        };
                        let Token::Identifier(class_name) = token_at.token else {
                            return Err(ParsingError::wrong_token(token_at, "class name"));
                        };
                        restrictions.push(SelectorRestriction::Class(class_name));
                    }
                    Token::OpenSquareBracket() => {
                        restrictions.push(SelectorRestriction::Attribute(parser.parse()?));
                    }
                    Token::Colon() => {
                        restrictions.push(SelectorRestriction::PseudoClass(parser.parse()?));
                    }
                    Token::Delimiter('*') => {
                        return Err(ParsingError::WrongToken {
                            line: token_at.line,
                            column: token_at.column,
                            expected: EXPECTED.to_owned(),
                            found: "*".to_owned(),
                        });
                    }
                    Token::Identifier(_) => {
                        return Err(ParsingError::WrongToken {
                            line: token_at.line,
                            column: token_at.column,
                            expected: EXPECTED.to_owned(),
                            found: "element".to_owned(),
                        });
                    }
                    _ => break,
                },
                None => break,
            }
        }

        let combinator = None;

        Ok(Selector {
            element,
            restrictions,
            combinator,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn universal() {
        let mut parser = Parser::new("*".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn element() {
        let mut parser = Parser::new("div".chars());
        assert_eq!(
            Ok(Selector {
                element: Some("div".to_string()),
                restrictions: vec![],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn id() {
        let mut parser = Parser::new("#id".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![SelectorRestriction::Id("id".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn class() {
        let mut parser = Parser::new(".class".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![SelectorRestriction::Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn attribute() {
        let mut parser = Parser::new("[key]".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![SelectorRestriction::Attribute(AttributeSelector::Exists(
                    "key".to_string()
                ))],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn pseudo_class() {
        let mut parser = Parser::new(":focus".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![SelectorRestriction::PseudoClass(PseudoClass::Focus)],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn element_with_class() {
        let mut parser = Parser::new("div.class".chars());
        assert_eq!(
            Ok(Selector {
                element: Some("div".to_string()),
                restrictions: vec![SelectorRestriction::Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn universal_with_class() {
        let mut parser = Parser::new("*.class".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![SelectorRestriction::Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn element_with_id_and_class() {
        let mut parser = Parser::new("div#id".chars());
        assert_eq!(
            Ok(Selector {
                element: Some("div".to_string()),
                restrictions: vec![SelectorRestriction::Id("id".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn three_classes() {
        let mut parser = Parser::new(".class1.class2.class3".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![
                    SelectorRestriction::Class("class1".to_string()),
                    SelectorRestriction::Class("class2".to_string()),
                    SelectorRestriction::Class("class3".to_string())
                ],
                combinator: None,
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.peek());
    }

    #[test]
    fn invalid_element() {
        let mut parser = Parser::new("123".chars());
        assert!(parser.parse::<Selector>().is_err());
    }

    #[test]
    fn invalid_id() {
        let mut parser = Parser::new("#123".chars());
        assert!(parser.parse::<Selector>().is_err());
    }

    #[test]
    fn invalid_class() {
        let mut parser = Parser::new(".123".chars());
        assert!(parser.parse::<Selector>().is_err());
    }

    #[test]
    fn invalid_attribute() {
        let mut parser = Parser::new("[123]".chars());
        assert!(parser.parse::<Selector>().is_err());
    }

    #[test]
    fn invalid_pseudo_class() {
        let mut parser = Parser::new(":123".chars());
        assert!(parser.parse::<Selector>().is_err());
    }

    #[test]
    fn class_before_universal() {
        let mut parser = Parser::new(".class*".chars());
        assert!(parser.parse::<Selector>().is_err());
    }
}
