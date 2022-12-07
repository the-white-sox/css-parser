use crate::parser::length::Length;

use super::*;

mod color_scheme;
mod hover;
mod orientation;
mod pointer;

pub use color_scheme::ColorScheme;
pub use hover::Hover;
pub use orientation::Orientation;
pub use pointer::Pointer;

#[derive(Debug, PartialEq)]
pub enum MediaFeature {
    Color,
    Monochrome,
    MinWidth(Length),
    Width(Length),
    MaxWidth(Length),
    MinHeight(Length),
    Height(Length),
    MaxHeight(Length),
    Orientation(Orientation),
    Hover(Hover),
    AnyHover(Hover),
    Pointer(Pointer),
    AnyPointer(Pointer),
    PrefersColorScheme(ColorScheme),
}

impl<I: Iterator<Item = char>> Parser<I> {
    fn consume_colon_separator(&mut self) -> Result<(), ParsingError> {
        self.optional_whitespace();
        self.expect(Token::Colon())?;
        self.optional_whitespace();
        Ok(())
    }
}

impl Parsable for MediaFeature {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(string) => match string.as_str() {
                    "color" => Ok(MediaFeature::Color),
                    "monochrome" => Ok(MediaFeature::Monochrome),
                    "min-width" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::MinWidth(length))
                    }
                    "width" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::Width(length))
                    }
                    "max-width" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::MaxWidth(length))
                    }
                    "min-height" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::MinHeight(length))
                    }
                    "height" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::Height(length))
                    }
                    "max-height" => {
                        parser.consume_colon_separator()?;
                        let length: Length = parser.parse()?;
                        Ok(MediaFeature::MaxHeight(length))
                    }
                    "orientation" => {
                        parser.consume_colon_separator()?;
                        let orientation: Orientation = parser.parse()?;
                        Ok(MediaFeature::Orientation(orientation))
                    }
                    "hover" => {
                        parser.consume_colon_separator()?;
                        let hover: Hover = parser.parse()?;
                        Ok(MediaFeature::Hover(hover))
                    }
                    "any-hover" => {
                        parser.consume_colon_separator()?;
                        let hover: Hover = parser.parse()?;
                        Ok(MediaFeature::AnyHover(hover))
                    }
                    "pointer" => {
                        parser.consume_colon_separator()?;
                        let pointer: Pointer = parser.parse()?;
                        Ok(MediaFeature::Pointer(pointer))
                    }
                    "any-pointer" => {
                        parser.consume_colon_separator()?;
                        let pointer: Pointer = parser.parse()?;
                        Ok(MediaFeature::AnyPointer(pointer))
                    }
                    "prefers-color-scheme" => {
                        parser.consume_colon_separator()?;
                        let color_scheme: ColorScheme = parser.parse()?;
                        Ok(MediaFeature::PrefersColorScheme(color_scheme))
                    }
                    _ => Err(ParsingError::wrong_token(token_at, "a media feature")),
                },

                _ => Err(ParsingError::wrong_token(token_at, "a media feature")),
            },

            None => Err(ParsingError::end_of_file("identifier")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::length::LengthUnit;

    use super::*;

    #[test]
    fn color() {
        let mut parser = Parser::new("color".chars());
        assert_eq!(Ok(MediaFeature::Color), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn monochrome() {
        let mut parser = Parser::new("monochrome".chars());
        assert_eq!(Ok(MediaFeature::Monochrome), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn min_width() {
        let mut parser = Parser::new("min-width: 100px".chars());
        assert_eq!(
            Ok(MediaFeature::MinWidth(Length::Length(
                100.0,
                LengthUnit::Pixels
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn width() {
        let mut parser = Parser::new("width: 240cm".chars());
        assert_eq!(
            Ok(MediaFeature::Width(Length::Length(
                240.0,
                LengthUnit::Centimeters
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn max_width() {
        let mut parser = Parser::new("max-width: 700pt".chars());
        assert_eq!(
            Ok(MediaFeature::MaxWidth(Length::Length(
                700.0,
                LengthUnit::Points
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn min_height() {
        let mut parser = Parser::new("min-height: 342em".chars());
        assert_eq!(
            Ok(MediaFeature::MinHeight(Length::Length(
                342.0,
                LengthUnit::FontSize
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn height() {
        let mut parser = Parser::new("height: 100vw".chars());
        assert_eq!(
            Ok(MediaFeature::Height(Length::Length(
                100.0,
                LengthUnit::ViewportWidth
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn max_height() {
        let mut parser = Parser::new("max-height: 150vmin".chars());
        assert_eq!(
            Ok(MediaFeature::MaxHeight(Length::Length(
                150.0,
                LengthUnit::ViewportMinimum
            ))),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn orientation() {
        let mut parser = Parser::new("orientation: portrait".chars());
        assert_eq!(
            Ok(MediaFeature::Orientation(Orientation::Portrait)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hover() {
        let mut parser = Parser::new("hover: hover".chars());
        assert_eq!(Ok(MediaFeature::Hover(Hover::Hover)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn any_hover() {
        let mut parser = Parser::new("any-hover: hover".chars());
        assert_eq!(Ok(MediaFeature::AnyHover(Hover::Hover)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn pointer() {
        let mut parser = Parser::new("pointer: fine".chars());
        assert_eq!(Ok(MediaFeature::Pointer(Pointer::Fine)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn any_pointer() {
        let mut parser = Parser::new("any-pointer: fine".chars());
        assert_eq!(Ok(MediaFeature::AnyPointer(Pointer::Fine)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn prefers_color_scheme() {
        let mut parser = Parser::new("prefers-color-scheme: dark".chars());
        assert_eq!(
            Ok(MediaFeature::PrefersColorScheme(ColorScheme::Dark)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn no_whitespace() {
        let mut parser = Parser::new("hover:hover".chars());
        assert_eq!(Ok(MediaFeature::Hover(Hover::Hover)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn extra_whitespace() {
        let mut parser = Parser::new("hover   :   hover".chars());
        assert_eq!(Ok(MediaFeature::Hover(Hover::Hover)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }
}
