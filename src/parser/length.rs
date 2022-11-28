use std::str::FromStr;

use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq)]
pub enum Distance {
    Zero(),
    Distance(f64, DistanceUnit),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DistanceUnit {
    Pixels,          // px
    Centimeters,     // cm
    Inches,          // in
    Points,          // pt
    FontSize,        // em
    RootFontSize,    // rem
    ViewportHeight,  // vh
    ViewportWidth,   // vw
    RootBlockSize,   // vb
    RootInlineSize,  // vi
    ViewportMinimum, // vmin
    ViewportMaximum, // vmax
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
            "vb" => Ok(RootBlockSize),
            "vi" => Ok(RootInlineSize),
            "vmin" => Ok(ViewportMinimum),
            "vmax" => Ok(ViewportMaximum),
            _ => Err(()),
        }
    }
}

impl Parsable for Distance {
    fn parse<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Self, ParsingError> {
        match parser.tokens.next() {
            Some(token_at) => match token_at.token {
                Token::Number(value) if value == 0.0 => Ok(Distance::Zero()),
                Token::Dimension(value, unit) => Ok(Distance::Distance(
                    value,
                    DistanceUnit::from_str(unit.as_str()).unwrap(),
                )),
                _ => Err(ParsingError::wrong_token(token_at, "dimension")),
            },
            None => Err(ParsingError::end_of_file("dimension")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // UNITS
    //

    #[test]
    fn unit_nothing() {
        let input = "";

        assert!(DistanceUnit::from_str(input).is_err());
    }

    #[test]
    fn unit_pixels() {
        let input = "px";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Pixels);
    }

    #[test]
    fn unit_centimeters() {
        let input = "cm";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Centimeters);
    }

    #[test]
    fn unit_inches() {
        let input = "in";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Inches);
    }

    #[test]
    fn unit_points() {
        let input = "pt";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Points);
    }

    #[test]
    fn unit_font_size() {
        let input = "em";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::FontSize);
    }

    #[test]
    fn unit_root_font_size() {
        let input = "rem";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootFontSize);
    }

    #[test]
    fn unit_viewport_height() {
        let input = "vh";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportHeight);
    }

    #[test]
    fn unit_viewport_width() {
        let input = "vw";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportWidth);
    }

    #[test]
    fn unit_root_block_size() {
        let input = "vb";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootBlockSize);
    }

    #[test]
    fn unit_root_inline_size() {
        let input = "vi";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootInlineSize);
    }

    #[test]
    fn unit_viewport_minimum() {
        let input = "vmin";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportMinimum);
    }

    #[test]
    fn unit_viewport_maximum() {
        let input = "vmax";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportMaximum);
    }

    #[test]
    fn invalid_unit() {
        let input = "xd";

        assert!(DistanceUnit::from_str(input).is_err());
    }

    //
    // DISTANCE
    //

    fn parse_distance(input: &str) -> Result<Distance, ParsingError> {
        let mut parser = Parser::new(input.chars());
        return parser.parse::<Distance>();
    }

    #[test]
    fn nothing() {
        assert!(parse_distance("").is_err());
    }

    #[test]
    fn only_unit() {
        assert!(parse_distance("vb").is_err());
    }

    #[test]
    fn positive_int_without_unit() {
        assert!(parse_distance("5").is_err());
    }

    #[test]
    fn negative_int_without_unit() {
        assert!(parse_distance("-69").is_err());
    }

    #[test]
    fn positive_float_without_unit() {
        assert!(parse_distance("66.6").is_err());
    }

    #[test]
    fn negative_float_without_unit() {
        assert!(parse_distance("-98.6").is_err());
    }

    #[test]
    fn zero() {
        let mut parser = Parser::new("0".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Zero());
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_int_with_unit() {
        let mut parser = Parser::new("23px".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Distance(23.0, DistanceUnit::Pixels));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_int_with_unit() {
        let mut parser = Parser::new("-394pt".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Distance(-394.0, DistanceUnit::Points));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_float_with_unit() {
        let mut parser = Parser::new("3.14rem".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Distance(3.14, DistanceUnit::RootFontSize));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_float_with_unit() {
        let mut parser = Parser::new("-1000000rem".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(
            result,
            Distance::Distance(-1000000.0, DistanceUnit::RootFontSize)
        );
        assert!(parser.tokens.next().is_none());
    }
}
