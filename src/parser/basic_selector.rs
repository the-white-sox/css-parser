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
                    expected: "valid Id".to_owned(),
                    found: id_name,
                }),
                Token::OpenSquareBracket() => todo!("add suport for attribute selectors"),
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
        let result = parser.parse::<BasicSelector>().unwrap();
        assert_eq!(result, BasicSelector::Universal())
    }
}
