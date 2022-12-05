// <sides-color> ::= <color> | <color> <color> | <color> <color> <color> <color>
// <color> ::= "black" | "silver" | "gray" | "grey" | "white" | "maroon" | "red" | "purple" | "fuchsia" | "green" | "lime" | "olive" | "yellow" | "navy" | "blue" | "teal" | "aqua" | <rgb> | <hsl> | <hex>
// <rgb> ::= "rgb(" <0-255> "," <0-255> "," <0-255> ")" | "rgba(" <0-255> "," <0-255> "," <0-255> "," <alpha> ")"
// <hex> ::= "#" <hex-byte> <hex-byte> <hex-byte> | "#" <hex-byte> <hex-byte> <hex-byte> <hex-byte> | "#" <hex-digit> <hex-digit> <hex-digit> | "#" <hex-digit> <hex-digit> <hex-digit> <hex-digit>
// <hex-digit> ::= <digit> | "a" - "f" | "A" - "F"
// <hex-byte> ::= <hex-digit> <hex-digit>
// <hsl> ::= "hsl(" <0-360> "," <0-100> "%," <0-100> "%)" | "hsla(" <0-360> "," <0-100> "%," <0-100> "%," <alpha> ")"
// <alpha> ::= "0." <digits> | "." <digits> | 1 | 0

use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    Silver,
    Gray,
    Grey,
    White,
    Maroon,
    Red,
    Purple,
    Fuchsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
    Rgb { r: f64, g: f64, b: f64, a: f64 },
    Hsl { h: f64, s: f64, l: f64, a: f64 },
}

fn parse_num<I: Iterator<Item = char>>(
    parser: &mut Parser<I>,
    min: f64,
    max: f64,
) -> Result<f64, ParsingError> {
    match parser.tokens.next() {
        Some(token_at) => match token_at.token {
            Token::Number(val) => {
                if val >= min && val <= max {
                    Ok(val)
                } else {
                    Err(ParsingError::wrong_token(
                        token_at,
                        &format!("a number between {min} and {max}"),
                    ))
                }
            }
            _ => Err(ParsingError::wrong_token(token_at, "a number")),
        },

        None => Err(ParsingError::end_of_file("a number")),
    }
}

impl Parsable for Color {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(key) => match key.to_lowercase().as_str() {
                    "black" => Ok(Color::Black),
                    "silver" => Ok(Color::Silver),
                    "gray" => Ok(Color::Gray),
                    "grey" => Ok(Color::Grey),
                    "white" => Ok(Color::White),
                    "maroon" => Ok(Color::Maroon),
                    "red" => Ok(Color::Red),
                    "purple" => Ok(Color::Purple),
                    "fuchsia" => Ok(Color::Fuchsia),
                    "green" => Ok(Color::Green),
                    "lime" => Ok(Color::Lime),
                    "olive" => Ok(Color::Olive),
                    "yellow" => Ok(Color::Yellow),
                    "navy" => Ok(Color::Navy),
                    "blue" => Ok(Color::Blue),
                    "teal" => Ok(Color::Teal),
                    "aqua" => Ok(Color::Aqua),
                    _ => Err(ParsingError::wrong_token(token_at, "black, silver, gray, grey, white, maroon, red, purple, fuchsia, green, lime, olive, yellow, navy, blue, teal, or aqua")),
                },
                Token::Function(name) => match name.to_lowercase().as_str() {
                    "rgb" => {
                        parser.optional_whitespace();
                        let r = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let g = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let b = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::CloseParenthesis())?;
                        Ok(Color::Rgb { r, g, b, a: 1.0 })
                    }
                    "rgba" => {
                        parser.optional_whitespace();
                        let r = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let g = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let b = parse_num(parser, 0.0, 255.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let a = parse_num(parser, 0.0, 1.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::CloseParenthesis())?;
                        Ok(Color::Rgb { r, g, b, a })
                    },
                    // "hsl" => {
                        
                    // },
                    _ => Err(ParsingError::wrong_token(token_at, "rgb, rgba, hex, hexa, hsl, or hsla")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "a color")),
            },

            None => Err(ParsingError::end_of_file("a color")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red() {
        let mut parser = Parser::new("red".chars());
        assert_eq!(Ok(Color::Red), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn black() {
        let mut parser = Parser::new("black".chars());
        assert_eq!(Ok(Color::Black), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn silver() {
        let mut parser = Parser::new("silver".chars());
        assert_eq!(Ok(Color::Silver), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn gray() {
        let mut parser = Parser::new("gray".chars());
        assert_eq!(Ok(Color::Gray), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn grey() {
        let mut parser = Parser::new("grey".chars());
        assert_eq!(Ok(Color::Grey), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn white() {
        let mut parser = Parser::new("white".chars());
        assert_eq!(Ok(Color::White), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn maroon() {
        let mut parser = Parser::new("maroon".chars());
        assert_eq!(Ok(Color::Maroon), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn purple() {
        let mut parser = Parser::new("purple".chars());
        assert_eq!(Ok(Color::Purple), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn fuchsia() {
        let mut parser = Parser::new("fuchsia".chars());
        assert_eq!(Ok(Color::Fuchsia), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn green() {
        let mut parser = Parser::new("green".chars());
        assert_eq!(Ok(Color::Green), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn lime() {
        let mut parser = Parser::new("lime".chars());
        assert_eq!(Ok(Color::Lime), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn olive() {
        let mut parser = Parser::new("olive".chars());
        assert_eq!(Ok(Color::Olive), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn yellow() {
        let mut parser = Parser::new("yellow".chars());
        assert_eq!(Ok(Color::Yellow), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn navy() {
        let mut parser = Parser::new("navy".chars());
        assert_eq!(Ok(Color::Navy), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn blue() {
        let mut parser = Parser::new("blue".chars());
        assert_eq!(Ok(Color::Blue), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn teal() {
        let mut parser = Parser::new("teal".chars());
        assert_eq!(Ok(Color::Teal), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn aqua() {
        let mut parser = Parser::new("aqua".chars());
        assert_eq!(Ok(Color::Aqua), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn pink() {
        let mut parser = Parser::new("pink".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn red_upper_case() {
        let mut parser = Parser::new("Red".chars());
        assert_eq!(Ok(Color::Red), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn rgb() {
        let mut parser = Parser::new("rgb( 37,102.4        ,0)".chars());
        assert_eq!(Ok(Color::Rgb { r:37.0, g:102.4, b:0.0, a:1.0}), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn rgb_no_comma() {
        let mut parser = Parser::new("rgb(102 5 23)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgb_out_of_upper_range() {
        let mut parser = Parser::new("rgb(300.0, 5.3, 23.0)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgb_out_of_lower_range() {
        let mut parser = Parser::new("rgb(50.0, -5.3, 23.0)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba() {
        let mut parser = Parser::new("rgba( 37,102.4        ,0,0.4)".chars());
        assert_eq!(Ok(Color::Rgb { r:37.0, g:102.4, b:0.0, a:0.4}), parser.parse());
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn rgba_out_of_lower_range() {
        let mut parser = Parser::new("rgb(50.0, 5.3, 23.0, -.4)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba_out_of_upper_range() {
        let mut parser = Parser::new("rgb(50.0, 5.3, 23.0, 500)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba_no_comma() {
        let mut parser = Parser::new("rgb(50.0, 5.3, 23.0 .4)".chars());
        assert!(parser.parse::<Color>().is_err());
    }
}
