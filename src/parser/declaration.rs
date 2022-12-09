// <declaration-list> ::= <declaration> ";" <declaration-list> | <declaration> | <declaration> ";"
// <declaration> ::= <color-property> ":" <color> | <sides-color-property> ":" <sides-color>
// <declaration> ::= <length-property> ":" <length-or-percentage> | <side-lengths-property> ":" <side-lengths>
// <declaration> ::= "font-family" ":" <string>
// <declaration> ::= "opacity" ":" <alpha>
// <declaration> ::= "text-align" ":" <text-align-value>
// <color-property> ::= "color" | "background-color"
// <sides-color-property> ::= "border-color"
// <length-property> ::= "font-size" | "height" | "width"
// <side-lengths-property> ::= "margin" | "padding" | "border-width" | "border-radius"

use super::{
    color::{parse_num, Color},
    font_family::FontFamily,
    length::Length,
    *,
};
use crate::tokenizer::*;

#[derive(Debug, PartialEq)]

pub enum Declaration {
    BackgroundColor(Color),
    BorderColor(Color),
    Opacity(f64),
    FontFamily(FontFamily),
    FontSize(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
    BorderWidth(Length),
    BorderRadius(Length),
}

impl Parsable for Declaration {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(key) => match key.as_str() {
                    "background-color" => {
                        parser.optional_whitespace();
                        parser.expect(Token::Colon())?;
                        parser.optional_whitespace();
                        Ok(Declaration::BackgroundColor(parser.parse()?))
                    }
                    "border-color" => {
                        parser.optional_whitespace();
                        parser.expect(Token::Colon())?;
                        parser.optional_whitespace();
                        Ok(Declaration::BorderColor(parser.parse()?))
                    }
                    "opacity" => {
                        parser.optional_whitespace();
                        parser.expect(Token::Colon())?;
                        parser.optional_whitespace();
                        let opacity = parse_num(parser, 0.0, 1.0)?;
                        Ok(Declaration::Opacity(opacity))
                    }
                    "font-family" => {
                        parser.optional_whitespace();
                        parser.expect(Token::Colon())?;
                        parser.optional_whitespace();
                        Ok(Declaration::FontFamily(parser.parse()?))
                    }

                    _ => Err(ParsingError::wrong_token(token_at, "a declaration")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "a declaration")),
            },
            _ => Err(ParsingError::end_of_file("a declaration")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_color() {
        let mut parser = Parser::new("background-color: red".chars());
        assert_eq!(Ok(Declaration::BackgroundColor(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_color() {
        let mut parser = Parser::new("border-color: red".chars());
        assert_eq!(Ok(Declaration::BorderColor(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn opacity() {
        let mut parser = Parser::new("opacity: 0.3".chars());
        assert_eq!(Ok(Declaration::Opacity(0.3)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn font_family() {
        let mut parser = Parser::new("font-family: Arial".chars());
        assert_eq!(
            Ok(Declaration::FontFamily(FontFamily(
                vec!["Arial".to_owned()]
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn font_size() {
        let mut parser = Parser::new("font-size: 3px".chars());
        assert_eq!(
            Ok(Declaration::FontSize(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn height() {
        let mut parser = Parser::new("height: 3px".chars());
        assert_eq!(
            Ok(Declaration::Height(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn width() {
        let mut parser = Parser::new("width: 3px".chars());
        assert_eq!(
            Ok(Declaration::Width(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn margin() {
        let mut parser = Parser::new("margin: 3px".chars());
        assert_eq!(
            Ok(Declaration::Margin(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn padding() {
        let mut parser = Parser::new("padding: 3px".chars());
        assert_eq!(
            Ok(Declaration::Padding(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_width() {
        let mut parser = Parser::new("border-width: 3px".chars());
        assert_eq!(
            Ok(Declaration::BorderWidth(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_radius() {
        let mut parser = Parser::new("border-radius: 3px".chars());
        assert_eq!(
            Ok(Declaration::BorderRadius(Length::Length(
                3.0,
                length::LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }
}
