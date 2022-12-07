use super::{color::*, *};

#[derive(Debug, PartialEq)]
pub enum SideColors {
    Single(Color),
    Double(Color, Color),
    Quad(Color, Color, Color, Color),
}

impl Parsable for SideColors {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        let first: Color = parser.parse()?;

        parser.optional_whitespace();

        match parser.tokens.peek() {
            Some(TokenAt {
                token: Token::Identifier(_) | Token::Function(_) | Token::Hash(_, _),
                ..
            }) => {
                // second color exists
            }
            _ => return Ok(SideColors::Single(first)),
        }

        let second: Color = parser.parse()?;

        parser.optional_whitespace();

        match parser.tokens.peek() {
            Some(TokenAt {
                token: Token::Identifier(_) | Token::Function(_) | Token::Hash(_, _),
                ..
            }) => {
                // third color exists
            }
            _ => return Ok(SideColors::Double(first, second)),
        }

        let third: Color = parser.parse()?;

        parser.optional_whitespace();

        let fourth: Color = parser.parse()?;

        Ok(SideColors::Quad(first, second, third, fourth))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_keyword() {
        let mut parser = Parser::new("red".chars());
        assert_eq!(Ok(SideColors::Single(Color::Red)), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_single_rgb() {
        let mut parser = Parser::new("rgb(255, 0, 0)".chars());
        assert_eq!(
            Ok(SideColors::Single(Color::Rgb {
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
            Ok(SideColors::Single(Color::Rgb {
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
        assert_eq!(
            Ok(SideColors::Double(Color::Red, Color::Blue)),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn test_double_rgb() {
        let mut parser = Parser::new("rgb(255, 0, 0) rgb(0, 0, 255)".chars());
        assert_eq!(
            Ok(SideColors::Double(
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
            Ok(SideColors::Double(
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
            Ok(SideColors::Quad(
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
            Ok(SideColors::Quad(
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
        assert!(parser.parse::<SideColors>().is_err());
    }

    #[test]
    fn five_colors() {
        let mut parser = Parser::new("red blue green yellow black".chars());
        assert_eq!(
            Ok(SideColors::Quad(
                Color::Red,
                Color::Blue,
                Color::Green,
                Color::Yellow
            )),
            parser.parse()
        );
        assert_ne!(None, parser.tokens.next());
    }
}
