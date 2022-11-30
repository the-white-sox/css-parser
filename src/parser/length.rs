use std::str::FromStr;

use super::*;
use crate::tokenizer::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Distance {
    Zero(),
    Distance(f64, DistanceUnit),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DistanceUnit {
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

impl FromStr for DistanceUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DistanceUnit::*;

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

impl Parsable for Distance {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match &token_at.token {
                Token::Number(value) if *value == 0.0 => Ok(Distance::Zero()),
                Token::Dimension(value, unit) => match DistanceUnit::from_str(unit.as_str()) {
                    Ok(unit) => Ok(Distance::Distance(*value, unit)),
                    Err(()) => Err(ParsingError::wrong_token(token_at, ""))
                },
                _ => Err(ParsingError::wrong_token(token_at, "dimension")),
            },
            None => Err(ParsingError::end_of_file("dimension")),
        }
    }
}
