use std::str::FromStr;

use super::{percentage::Percentage, *};
use crate::tokenizer::*;

#[cfg(test)]
mod tests;

/// Represents a single length value.
/// There are two types: zero (with no unit), and a number with a unit
/// Grammar: `<length>`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Length {
    Zero(),
    Length(f64, LengthUnit),
}

/// Represents a length unit.
/// Grammar: `<length-unit>`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

/// Wraps a length or percentage together
/// Grammar: `<length-or-percentage>
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(Percentage),
}

/// Wrapper for side lengths ammounts of 1, 2, and 4.
/// Grammar: `<side-length>`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SideLength {
    Single(LengthOrPercentage),
    Double(LengthOrPercentage, LengthOrPercentage),
    Quad(
        LengthOrPercentage,
        LengthOrPercentage,
        LengthOrPercentage,
        LengthOrPercentage,
    ),
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
                    Err(()) => Err(ParsingError::wrong_token(
                        token_at,
                        "one of px, cm, in, pt, em, rem, vh, vw, vb, vi, vmin, or vmax",
                    )),
                },
                _ => Err(ParsingError::wrong_token(token_at, "dimension")),
            },
            None => Err(ParsingError::end_of_file("dimension")),
        }
    }
}

impl Parsable for LengthOrPercentage {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.peek() {
            Some(token_at) => match token_at.token {
                // parse length
                Token::Number(_) | Token::Dimension(_, _) => match parser.parse::<Length>() {
                    Ok(length) => Ok(LengthOrPercentage::Length(length)),
                    Err(error) => Err(error),
                },

                // parse percentage
                Token::Percentage(_) => match parser.parse::<Percentage>() {
                    Ok(percentage) => Ok(LengthOrPercentage::Percentage(percentage)),
                    Err(error) => Err(error),
                },

                // neither
                _ => Err(ParsingError::wrong_token(
                    token_at.clone(),
                    "length or percentage",
                )),
            },
            None => Err(ParsingError::end_of_file("length or percentage")),
        }
    }
}

impl Parsable for SideLength {
    fn parse<I: Iterator<Item = char>>(_: &mut Parser<I>) -> Result<Self, ParsingError> {
        todo!();
    }
}
