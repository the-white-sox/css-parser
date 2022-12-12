pub mod attribute_selector;
pub mod combinator;
pub mod pseudo_class;
pub mod relative_selector;

use attribute_selector::AttributeSelector;
use combinator::Combinator;
use pseudo_class::PseudoClass;

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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use SelectorRestriction::*;

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
    }

    #[test]
    fn id() {
        let mut parser = Parser::new("#id".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![Id("id".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn class() {
        let mut parser = Parser::new(".class".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn attribute() {
        let mut parser = Parser::new("[key]".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![Attribute(AttributeSelector::Exists("key".to_string()))],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn element_with_class() {
        let mut parser = Parser::new("div.class".chars());
        assert_eq!(
            Ok(Selector {
                element: Some("div".to_string()),
                restrictions: vec![Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn universal_with_class() {
        let mut parser = Parser::new("*.class".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![Class("class".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn element_with_id_and_class() {
        let mut parser = Parser::new("div#id".chars());
        assert_eq!(
            Ok(Selector {
                element: Some("div".to_string()),
                restrictions: vec![Id("id".to_string())],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn three_classes() {
        let mut parser = Parser::new(".class1.class2.class3".chars());
        assert_eq!(
            Ok(Selector {
                element: None,
                restrictions: vec![
                    Class("class1".to_string()),
                    Class("class2".to_string()),
                    Class("class3".to_string())
                ],
                combinator: None,
            }),
            parser.parse()
        );
    }

    #[test]
    fn invalid_element() {
        let mut parser = Parser::new("123".chars());
        assert!(parser.parse::<Selector>().is_err(),);
    }

    #[test]
    fn invalid_id() {
        let mut parser = Parser::new("#123".chars());
        assert!(parser.parse::<Selector>().is_err(),);
    }

    #[test]
    fn invalid_class() {
        let mut parser = Parser::new(".123".chars());
        assert!(parser.parse::<Selector>().is_err(),);
    }

    #[test]
    fn invalid_attribute() {
        let mut parser = Parser::new("[123]".chars());
        assert!(parser.parse::<Selector>().is_err(),);
    }

    #[test]
    fn class_before_universal() {
        let mut parser = Parser::new(".class*".chars());
        assert!(parser.parse::<Selector>().is_err(),);
    }
}
