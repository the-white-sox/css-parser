pub mod text_align;
mod vec;

use super::{
    color::{parse_num, Color},
    font_family::FontFamily,
    length::Length,
    length_or_percentage::LengthOrPercentage,
    side::Sides,
    *,
};
use crate::tokenizer::*;
use text_align::TextAlign;
pub use vec::*;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    BackgroundColor(Color),
    BorderColor(Color),
    Opacity(f64),
    FontFamily(FontFamily),
    FontSize(Length),
    Height(Length),
    Width(Length),
    Margin(Sides<LengthOrPercentage>),
    Padding(Sides<LengthOrPercentage>),
    BorderWidth(Sides<LengthOrPercentage>),
    BorderRadius(Sides<LengthOrPercentage>),
    TextAlign(TextAlign),
}

impl Parsable for Declaration {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(key) => match key.as_str() {
                    "background-color" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::BackgroundColor(parser.parse()?))
                    }
                    "border-color" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::BorderColor(parser.parse()?))
                    }
                    "opacity" => {
                        parser.consume_colon_separator()?;
                        let opacity = parse_num(parser, 0.0, 1.0)?;
                        Ok(Declaration::Opacity(opacity))
                    }
                    "font-family" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::FontFamily(parser.parse()?))
                    }
                    "font-size" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::FontSize(parser.parse()?))
                    }
                    "height" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Height(parser.parse()?))
                    }
                    "width" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Width(parser.parse()?))
                    }
                    "margin" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Margin(parser.parse()?))
                    }
                    "padding" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Padding(parser.parse()?))
                    }
                    "border-width" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::BorderWidth(parser.parse()?))
                    }
                    "border-radius" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::BorderRadius(parser.parse()?))
                    }
                    "text-align" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::TextAlign(parser.parse()?))
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
    use super::{length::LengthUnit, *};

    #[test]
    fn background_color() {
        let mut parser = Parser::new("background-color: red".chars());
        assert_eq!(Ok(Declaration::BackgroundColor(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn background_color_garbage() {
        let mut parser = Parser::new("background-color: aifdsn".chars());
        assert!(parser.parse::<Declaration>().is_err());
    }

    #[test]
    fn border_color() {
        let mut parser = Parser::new("border-color: red".chars());
        assert_eq!(Ok(Declaration::BorderColor(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_color_garbage() {
        let mut parser = Parser::new("border-color: aifdsn".chars());
        assert!(parser.parse::<Declaration>().is_err());
    }

    #[test]
    fn opacity() {
        let mut parser = Parser::new("opacity: 0.3".chars());
        assert_eq!(Ok(Declaration::Opacity(0.3)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn opacity_negative() {
        let mut parser = Parser::new("opacity: -4.3".chars());
        assert!(parser.parse::<Declaration>().is_err());
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
    fn text_align() {
        let mut parser = Parser::new("text-align: center".chars());
        assert_eq!(
            Ok(Declaration::TextAlign(TextAlign::Center)),
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
                LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn height() {
        let mut parser = Parser::new("height: 3px".chars());
        assert_eq!(
            Ok(Declaration::Height(Length::Length(3.0, LengthUnit::Pixels))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn width() {
        let mut parser = Parser::new("width: 3px".chars());
        assert_eq!(
            Ok(Declaration::Width(Length::Length(3.0, LengthUnit::Pixels))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn margin() {
        let mut parser = Parser::new("margin: 3px".chars());
        assert_eq!(
            Ok(Declaration::Margin(Sides::Single(
                LengthOrPercentage::Length(Length::Length(3.0, LengthUnit::Pixels))
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn padding() {
        let mut parser = Parser::new("padding: 3px".chars());
        assert_eq!(
            Ok(Declaration::Padding(Sides::Single(
                LengthOrPercentage::Length(Length::Length(3.0, LengthUnit::Pixels))
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_width() {
        let mut parser = Parser::new("border-width: 3px".chars());
        assert_eq!(
            Ok(Declaration::BorderWidth(Sides::Single(
                LengthOrPercentage::Length(Length::Length(3.0, LengthUnit::Pixels))
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_radius() {
        let mut parser = Parser::new("border-radius: 3px".chars());
        assert_eq!(
            Ok(Declaration::BorderRadius(Sides::Single(
                LengthOrPercentage::Length(Length::Length(3.0, LengthUnit::Pixels))
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn bad_declaration() {
        let mut parser = Parser::new("band-color: red".chars());
        assert!(parser.parse::<Declaration>().is_err());
    }
}
