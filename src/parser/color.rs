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

impl Parsable for Color {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let mut parser = Parser::new("url(example.com)".chars());
        let result = parser.parse::<Url>().unwrap();
        assert_eq!(
            result,
            Url {
                url: "example.com".to_owned()
            }
        );
        assert!(parser.tokens.next().is_none());
    }
}
