use std::str::FromStr;

use super::*;
use crate::tokenizer::*;

#[cfg(test)]
mod tests;

/// Represents a single length value.
/// There are two types: zero (with no unit), and a number with a unit
/// Grammar: `<length>`
#[derive(Debug, PartialEq)]
pub enum Length {
    Zero(),
    Length(f64, LengthUnit),
}

/// Represents a length unit.
/// Grammar: `<length-unit>`
#[derive(Debug, PartialEq, Eq)]
pub enum LengthUnit {
    Pixels,             // px
    Centimeters,        // cm
    Inches,             // in
    Points,             // pt
    FontSize,           // em
    RootFontSize,       // rem
    ViewportHeight,     // vh
    ViewportWidth,      // vw
    ViewportBlockSize,  // vb
    ViewportInlineSize, // vi
    ViewportMinimum,    // vmin
    ViewportMaximum,    // vmax
}

impl FromStr for LengthUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LengthUnit::*;

        match s {
            "px" => Ok(Pixels),
            "cm" => Ok(Centimeters),
            "in" => Ok(Inches),
            "pt" => Ok(Points),
            "em" => Ok(FontSize),
            "rem" => Ok(RootFontSize),
            "vh" => Ok(ViewportHeight),
            "vw" => Ok(ViewportWidth),
            "vb" => Ok(ViewportBlockSize),
            "vi" => Ok(ViewportInlineSize),
            "vmin" => Ok(ViewportMinimum),
            "vmax" => Ok(ViewportMaximum),
            _ => Err(()),
        }
    }
}

impl Parsable for Length {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Number(value) if *value == 0.0 => Ok(Length::Zero()),
                Token::Dimension(value, unit) => match unit.parse::<LengthUnit>() {
                    Ok(unit) => Ok(Length::Length(*value, unit)),
                    Err(()) => Err(ParsingError::wrong_token(token_at, "dimension")),
                },
                _ => Err(ParsingError::wrong_token(token_at, "dimension")),
            },
            None => Err(ParsingError::end_of_file("dimension")),
        }
    }
}
