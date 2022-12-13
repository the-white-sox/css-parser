pub mod display;
pub mod position;
pub mod text_align;
mod vec;

use super::{
    color::{parse_num, Color},
    font_family::FontName,
    length_or_percentage::LengthOrPercentage,
    side::Sides,
    *,
};
use crate::tokenizer::*;
use display::Display;
use position::Position;
use text_align::TextAlign;
pub use vec::*;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    BackgroundColor(Color),
    BorderColor(Sides<Color>),
    Opacity(f64),
    FontFamily(Vec<FontName>),
    FontSize(LengthOrPercentage),
    MinHeight(LengthOrPercentage),
    Height(LengthOrPercentage),
    MaxHeight(LengthOrPercentage),
    MinWidth(LengthOrPercentage),
    Width(LengthOrPercentage),
    MaxWidth(LengthOrPercentage),
    Margin(Sides<LengthOrPercentage>),
    Padding(Sides<LengthOrPercentage>),
    BorderWidth(Sides<LengthOrPercentage>),
    BorderRadius(Sides<LengthOrPercentage>),
    TextAlign(TextAlign),
    Color(Color),
    Display(Display),
    Position(Position),
    Top(LengthOrPercentage),
    Bottom(LengthOrPercentage),
    Left(LengthOrPercentage),
    Right(LengthOrPercentage),
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
                    "min-height" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::MinHeight(parser.parse()?))
                    }
                    "height" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Height(parser.parse()?))
                    }
                    "max-height" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::MaxHeight(parser.parse()?))
                    }
                    "min-width" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::MinWidth(parser.parse()?))
                    }
                    "width" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Width(parser.parse()?))
                    }
                    "max-width" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::MaxWidth(parser.parse()?))
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
                    "color" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Color(parser.parse()?))
                    }
                    "display" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Display(parser.parse()?))
                    }
                    "position" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Position(parser.parse()?))
                    }
                    "top" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Top(parser.parse()?))
                    }
                    "bottom" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Bottom(parser.parse()?))
                    }
                    "left" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Left(parser.parse()?))
                    }
                    "right" => {
                        parser.consume_colon_separator()?;
                        Ok(Declaration::Right(parser.parse()?))
                    }

                    _ => Err(ParsingError::wrong_token(token_at, "a valid property name")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "a valid property name")),
            },
            _ => Err(ParsingError::end_of_file("a valid property name")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{length::Length, length::LengthUnit, percentage::Percentage, *};

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
        assert_eq!(
            Ok(Declaration::BorderColor(Sides::Single(Color::Red))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn border_color_garbage() {
        let mut parser = Parser::new("border-color: aifdsn".chars());
        assert!(parser.parse::<Declaration>().is_err());
    }

    #[test]
    fn quad_border_color() {
        let mut parser = Parser::new("border-color: red green blue rgb(10, 20, 30)".chars());
        assert_eq!(
            Ok(Declaration::BorderColor(Sides::Quad(
                Color::Red,
                Color::Green,
                Color::Blue,
                Color::Rgb {
                    r: 10.0,
                    g: 20.0,
                    b: 30.0,
                    a: 1.0
                }
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
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
            Ok(Declaration::FontFamily(vec![FontName("Arial".to_owned())])),
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
            Ok(Declaration::FontSize(LengthOrPercentage::Length(
                Length::Length(3.0, LengthUnit::Pixels)
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn height() {
        let mut parser = Parser::new("height: 3px".chars());
        assert_eq!(
            Ok(Declaration::Height(LengthOrPercentage::Length(
                Length::Length(3.0, LengthUnit::Pixels)
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn width() {
        let mut parser = Parser::new("width: 3px".chars());
        assert_eq!(
            Ok(Declaration::Width(LengthOrPercentage::Length(
                Length::Length(3.0, LengthUnit::Pixels)
            ))),
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
    fn quad_margin() {
        let mut parser = Parser::new("margin: 3px 20% 700rem -100pt".chars());
        assert_eq!(
            Ok(Declaration::Margin(Sides::Quad(
                LengthOrPercentage::Length(Length::Length(3.0, LengthUnit::Pixels)),
                LengthOrPercentage::Percentage(Percentage(20.0)),
                LengthOrPercentage::Length(Length::Length(700.0, LengthUnit::RootFontSize)),
                LengthOrPercentage::Length(Length::Length(-100.0, LengthUnit::Points)),
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
    fn display() {
        let mut parser = Parser::new("display: block".chars());
        assert_eq!(Ok(Declaration::Display(Display::Block)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn position() {
        let mut parser = Parser::new("position: absolute".chars());
        assert_eq!(
            Ok(Declaration::Position(Position::Absolute)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn top() {
        let mut parser = Parser::new("top: 0".chars());
        assert_eq!(
            Ok(Declaration::Top(LengthOrPercentage::Length(Length::Zero()))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn bottom() {
        let mut parser = Parser::new("bottom: 0".chars());
        assert_eq!(
            Ok(Declaration::Bottom(LengthOrPercentage::Length(
                Length::Zero()
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn left() {
        let mut parser = Parser::new("left: 0".chars());
        assert_eq!(
            Ok(Declaration::Left(
                LengthOrPercentage::Length(Length::Zero())
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn right() {
        let mut parser = Parser::new("right: 0".chars());
        assert_eq!(
            Ok(Declaration::Right(LengthOrPercentage::Length(
                Length::Zero()
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

    #[test]
    fn color() {
        let mut parser = Parser::new("color: red".chars());
        assert_eq!(Ok(Declaration::Color(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn color_garbage() {
        let mut parser = Parser::new("color: awrrvads".chars());
        assert!(parser.parse::<Declaration>().is_err());
    }
}
