use super::{side::CanStart, *};
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
    Transparent,
    Rgb { r: f64, g: f64, b: f64, a: f64 },
    Hsl { h: f64, s: f64, l: f64, a: f64 },
}

fn parse_percent<I: Iterator<Item = char>>(
    parser: &mut Parser<I>,
    min: f64,
    max: f64,
) -> Result<f64, ParsingError> {
    match parser.tokens.next() {
        Some(token_at) => match token_at.token {
            Token::Percentage(val) => {
                if val >= min && val <= max {
                    Ok(val)
                } else {
                    Err(ParsingError::wrong_token(
                        token_at,
                        &format!("a percentage between {min}% and {max}%"),
                    ))
                }
            }
            _ => Err(ParsingError::wrong_token(token_at, "a percentage")),
        },

        None => Err(ParsingError::end_of_file("a percentage")),
    }
}

pub fn parse_num<I: Iterator<Item = char>>(
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

/// doubles each character in a string
fn double_hex_characters(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        output.push(char);
        output.push(char);
    }
    output
}

impl Parsable for Color {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Identifier(key) => match key.as_str() {
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
                    "transparent" => Ok(Color::Transparent),
                    _ => Err(ParsingError::wrong_token(token_at, "black, silver, gray, grey, white, maroon, red, purple, fuchsia, green, lime, olive, yellow, navy, blue, teal, aqua, or transparent")),
                },
                Token::Function(name) => match name.as_str() {
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
                    "hsl" => {
                        parser.optional_whitespace();
                        let h = parse_num(parser, 0.0, 360.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let s = parse_percent(parser, 0.0, 100.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let l = parse_percent(parser, 0.0, 100.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::CloseParenthesis())?;
                        Ok(Color::Hsl { h, s, l, a: 1.0 })
                    },
                    "hsla" => {
                        parser.optional_whitespace();
                        let h = parse_num(parser, 0.0, 360.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let s = parse_percent(parser, 0.0, 100.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let l = parse_percent(parser, 0.0, 100.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::Comma())?;
                        parser.optional_whitespace();
                        let a = parse_num(parser, 0.0, 1.0)?;
                        parser.optional_whitespace();
                        parser.expect(Token::CloseParenthesis())?;
                        Ok(Color::Hsl { h, s, l, a })
                    },
                    _ => Err(ParsingError::wrong_token(token_at, "rgb, rgba, hsl, or hsla")),
                },
                Token::Hash(value, _) => {
                    let value = match value.len() {
                        3 | 4 => double_hex_characters(value),
                        6 | 8 => value.clone(),
                        _ => return Err(ParsingError::wrong_token(token_at, "3, 4, 6, or 8 character long hex value")),
                    };

                    let Ok(r) = u8::from_str_radix(&value[0..2], 16) else {
                        return Err(ParsingError::wrong_token(token_at, "a valid hex value"));
                    };

                    let Ok(g) = u8::from_str_radix(&value[2..4], 16) else {
                        return Err(ParsingError::wrong_token(token_at, "a valid hex value"));
                    };

                    let Ok(b) = u8::from_str_radix(&value[4..6], 16) else {
                        return Err(ParsingError::wrong_token(token_at, "a valid hex value"));
                    };

                    let a = if value.len() == 8 {
                        let Ok(a) = u8::from_str_radix(&value[6..8], 16) else {
                            return Err(ParsingError::wrong_token(token_at, "a valid hex value"));
                        };
                        a as f64 / 255.0
                    } else {
                        1.0
                    };

                    Ok(Color::Rgb { r: r as f64, g: g as f64, b: b as f64, a})
                }
                _ => Err(ParsingError::wrong_token(token_at, "a color")),
            },

            None => Err(ParsingError::end_of_file("a color")),
        }
    }
}

impl CanStart for Color {
    fn can_start(token: &Token) -> bool {
        matches!(
            token,
            Token::Identifier(_) | Token::Function(_) | Token::Hash(_, _)
        )
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
    fn transparent() {
        let mut parser = Parser::new("transparent".chars());
        assert_eq!(Ok(Color::Transparent), parser.parse());
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
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgb() {
        let mut parser = Parser::new("rgb( 37,102.4        ,0)".chars());
        assert_eq!(
            Ok(Color::Rgb {
                r: 37.0,
                g: 102.4,
                b: 0.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn rgb_no_comma() {
        let mut parser = Parser::new("rgb(102 5 23)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgb_uppcase() {
        let mut parser = Parser::new("RGB(50.0, 5.3, 23.0)".chars());
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
        assert_eq!(
            Ok(Color::Rgb {
                r: 37.0,
                g: 102.4,
                b: 0.0,
                a: 0.4
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn rgba_out_of_lower_range() {
        let mut parser = Parser::new("rgba(50.0, 5.3, 23.0, -.4)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba_uppcase() {
        let mut parser = Parser::new("RGBA(50.0, 5.3, 23.0, 0.5)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba_out_of_upper_range() {
        let mut parser = Parser::new("rgba(50.0, 5.3, 23.0, 500)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn rgba_no_comma() {
        let mut parser = Parser::new("rgba(50.0, 5.3, 23.0 .4)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsl() {
        let mut parser = Parser::new("hsl( 37,99.4%        ,0%)".chars());
        assert_eq!(
            Ok(Color::Hsl {
                h: 37.0,
                s: 99.4,
                l: 0.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hsl_no_comma() {
        let mut parser = Parser::new("hsl(50.0, 5.3% 23.0%)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsl_uppcase() {
        let mut parser = Parser::new("HSL(50.0, 5.3%, 23.0%)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsl_out_of_upper_range() {
        let mut parser = Parser::new("hsl(50.0, 105.3%, 23.0%)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsl_out_of_lower_range() {
        let mut parser = Parser::new("hsl(50.0, 5.3%, -23.0%)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsla() {
        let mut parser = Parser::new("hsla( 37,99.4%        ,0%, 0.4)".chars());
        assert_eq!(
            Ok(Color::Hsl {
                h: 37.0,
                s: 99.4,
                l: 0.0,
                a: 0.4
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hsla_no_comma() {
        let mut parser = Parser::new("hsla(50.0, 5.3% 23.0%, 0.6)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsla_uppcase() {
        let mut parser = Parser::new("HSLA(50.0, 5.3%, 23.0%,0.7)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsla_out_of_upper_range() {
        let mut parser = Parser::new("hsla(50.0, 1.3%, 23.0%, 65.6)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hsla_out_of_lower_range() {
        let mut parser = Parser::new("hsla(50.0, 5.3%, 3.0%,-3)".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn double_hex_characters_helper_functions() {
        assert_eq!("112233", double_hex_characters("123"));
        assert_eq!("aabbcc", double_hex_characters("abc"));
        assert_eq!("11223344", double_hex_characters("1234"));
    }

    #[test]
    fn hex_6() {
        let mut parser = Parser::new("#F4AA31".chars());
        assert_eq!(
            Ok(Color::Rgb {
                r: 244.0,
                g: 170.0,
                b: 49.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hex_4() {
        let mut parser = Parser::new("#123F".chars());
        assert_eq!(
            Ok(Color::Rgb {
                r: 17.0,
                g: 34.0,
                b: 51.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hex_3() {
        let mut parser = Parser::new("#123".chars());
        assert_eq!(
            Ok(Color::Rgb {
                r: 17.0,
                g: 34.0,
                b: 51.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hex_8() {
        let mut parser = Parser::new("#112233ff".chars());
        assert_eq!(
            Ok(Color::Rgb {
                r: 17.0,
                g: 34.0,
                b: 51.0,
                a: 1.0
            }),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn hex_10() {
        let mut parser = Parser::new("#FF48FA6EBA".chars());
        assert!(parser.parse::<Color>().is_err());
    }

    #[test]
    fn hex_out_of_range() {
        let mut parser = Parser::new("#H8NKNC".chars());
        assert!(parser.parse::<Color>().is_err());
    }
}
