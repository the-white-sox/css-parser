use super::*;

#[derive(Debug, PartialEq)]
pub enum Sides<T> {
    Single(T),
    Double(T, T),
    Quad(T, T, T, T),
}

impl<T: CanStart> Parsable for Sides<T> {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let first: T = parser.parse()?;

        parser.optional_whitespace();

        match parser.tokens.peek() {
            Some(token_at) if T::can_start(&token_at.token) => {
                // second color exists
            }
            _ => return Ok(Sides::Single(first)),
        }

        let second: T = parser.parse()?;

        parser.optional_whitespace();

        match parser.tokens.peek() {
            Some(token_at) if T::can_start(&token_at.token) => {
                // third color exists
            }
            _ => return Ok(Sides::Double(first, second)),
        }

        let third: T = parser.parse()?;

        parser.optional_whitespace();

        let fourth: T = parser.parse()?;

        Ok(Sides::Quad(first, second, third, fourth))
    }
}

pub trait CanStart: Parsable {
    fn can_start(token: &Token) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::parser::length::{Length, LengthOrPercentage};

    use super::{color::Color, *};

    #[test]
    fn test_single_keyword() {
        let mut parser = Parser::new("red".chars());
        assert_eq!(Ok(Sides::Single(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_single_rgb() {
        let mut parser = Parser::new("rgb(255, 0, 0)".chars());
        assert_eq!(
            Ok(Sides::Single(Color::Rgb {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            })),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_single_hex() {
        let mut parser = Parser::new("#ff0000".chars());
        assert_eq!(
            Ok(Sides::Single(Color::Rgb {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            })),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_double_keyword() {
        let mut parser = Parser::new("red blue".chars());
        assert_eq!(Ok(Sides::Double(Color::Red, Color::Blue)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_double_rgb() {
        let mut parser = Parser::new("rgb(255, 0, 0) rgb(0, 0, 255)".chars());
        assert_eq!(
            Ok(Sides::Double(
                Color::Rgb {
                    r: 255.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0
                },
                Color::Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 255.0,
                    a: 1.0
                }
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_double_hex() {
        let mut parser = Parser::new("#ff0000 #0000ff".chars());
        assert_eq!(
            Ok(Sides::Double(
                Color::Rgb {
                    r: 255.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0
                },
                Color::Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 255.0,
                    a: 1.0
                }
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_quad_keyword() {
        let mut parser = Parser::new("red blue green yellow".chars());
        assert_eq!(
            Ok(Sides::Quad(
                Color::Red,
                Color::Blue,
                Color::Green,
                Color::Yellow
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn mixed_colors() {
        let mut parser = Parser::new("red rgb(0, 0, 255) #00ff00 yellow".chars());
        assert_eq!(
            Ok(Sides::Quad(
                Color::Red,
                Color::Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 255.0,
                    a: 1.0
                },
                Color::Rgb {
                    r: 0.0,
                    g: 255.0,
                    b: 0.0,
                    a: 1.0
                },
                Color::Yellow
            )),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn three_colors() {
        let mut parser = Parser::new("red blue green".chars());
        assert!(parser.parse::<Sides<Color>>().is_err());
    }

    #[test]
    fn five_colors() {
        let mut parser = Parser::new("red blue green yellow black".chars());
        assert_eq!(
            Ok(Sides::Quad(
                Color::Red,
                Color::Blue,
                Color::Green,
                Color::Yellow
            )),
            parser.parse()
        );
        assert_ne!(None, parser.tokens.next());
    }

    #[test]
    fn one_zero() {
        let mut parser = Parser::new("0".chars());
        assert_eq!(
            Ok(Sides::Single(LengthOrPercentage::Length(Length::Zero()))),
            parser.parse(),
        );
    }

    #[test]
    fn two_zeros() {
        let mut parser = Parser::new("0 0".chars());
        assert_eq!(
            Ok(Sides::Double(
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero())
            )),
            parser.parse(),
        );
    }

    #[test]
    fn three_zeros() {
        let mut parser = Parser::new("0 0 0".chars());
        assert!(parser.parse::<Sides<LengthOrPercentage>>().is_err());
    }

    #[test]
    fn four_zeros() {
        let mut parser = Parser::new("0 0 0 0".chars());
        assert_eq!(
            Ok(Sides::Quad(
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero())
            )),
            parser.parse(),
        );
    }

    #[test]
    fn five_zeros() {
        let mut parser = Parser::new("0 0 0 0 0".chars());
        assert_eq!(
            Ok(Sides::Quad(
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero()),
                LengthOrPercentage::Length(Length::Zero())
            )),
            parser.parse(),
        );
        assert_ne!(None, parser.tokens.next());
    }
}
