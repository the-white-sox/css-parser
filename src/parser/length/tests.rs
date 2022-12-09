use super::*;

mod side_length {
    use crate::parser::{length_or_percentage::LengthOrPercentage, side::Sides};

    use super::*;

    fn parse_side_length(input: &str) -> Result<Sides<LengthOrPercentage>, ParsingError> {
        let mut parser = Parser::new(input.chars());
        return parser.parse();
    }

    mod unit {
        use super::*;

        fn parse_unit(input: &str) -> Result<LengthUnit, ()> {
            return input.parse::<LengthUnit>();
        }

        #[test]
        fn nothing() {
            assert!(parse_unit("").is_err());
        }

        #[test]
        fn pixels() {
            assert_eq!(parse_unit("px"), Ok(LengthUnit::Pixels));
        }

        #[test]
        fn centimeters() {
            assert_eq!(parse_unit("cm"), Ok(LengthUnit::Centimeters));
        }

        #[test]
        fn inches() {
            assert_eq!(parse_unit("in"), Ok(LengthUnit::Inches));
        }

        #[test]
        fn points() {
            assert_eq!(parse_unit("pt"), Ok(LengthUnit::Points));
        }

        #[test]
        fn font_size() {
            assert_eq!(parse_unit("em"), Ok(LengthUnit::FontSize));
        }

        #[test]
        fn root_font_size() {
            assert_eq!(parse_unit("rem"), Ok(LengthUnit::RootFontSize));
        }

        #[test]
        fn viewport_height() {
            assert_eq!(parse_unit("vh"), Ok(LengthUnit::ViewportHeight));
        }

        #[test]
        fn viewport_width() {
            assert_eq!(parse_unit("vw"), Ok(LengthUnit::ViewportWidth));
        }

        #[test]
        fn root_block_size() {
            assert_eq!(parse_unit("vb"), Ok(LengthUnit::ViewportBlockSize));
        }

        #[test]
        fn root_inline_size() {
            assert_eq!(parse_unit("vi"), Ok(LengthUnit::ViewportInlineSize));
        }

        #[test]
        fn viewport_minimum() {
            assert_eq!(parse_unit("vmin"), Ok(LengthUnit::ViewportMinimum));
        }

        #[test]
        fn viewport_maximum() {
            assert_eq!(parse_unit("vmax"), Ok(LengthUnit::ViewportMaximum));
        }

        #[test]
        fn invalid_unit() {
            assert!(parse_unit("xd").is_err());
        }
    }

    mod single {
        use super::*;

        fn parse_single_length(input: &str) -> Result<Length, ParsingError> {
            let mut parser = Parser::new(input.chars());
            return parser.parse::<Length>();
        }

        #[test]
        fn nothing() {
            assert!(parse_single_length("").is_err());
        }

        #[test]
        fn only_unit() {
            assert!(parse_single_length("vb").is_err());
        }

        #[test]
        fn positive_int_without_unit() {
            assert!(parse_single_length("5").is_err());
        }

        #[test]
        fn negative_int_without_unit() {
            assert!(parse_single_length("-69").is_err());
        }

        #[test]
        fn positive_float_without_unit() {
            assert!(parse_single_length("66.6").is_err());
        }

        #[test]
        fn negative_float_without_unit() {
            assert!(parse_single_length("-98.6").is_err());
        }

        #[test]
        fn zero() {
            let mut parser = Parser::new("0".chars());
            let result = parser.parse::<Length>().unwrap();

            assert_eq!(result, Length::Zero());
            assert!(parser.tokens.next().is_none());
        }

        #[test]
        fn positive_int_with_unit() {
            let mut parser = Parser::new("23px".chars());
            let result = parser.parse::<Length>().unwrap();

            assert_eq!(result, Length::Length(23.0, LengthUnit::Pixels));
            assert!(parser.tokens.next().is_none());
        }

        #[test]
        fn negative_int_with_unit() {
            let mut parser = Parser::new("-394pt".chars());
            let result = parser.parse::<Length>().unwrap();

            assert_eq!(result, Length::Length(-394.0, LengthUnit::Points));
            assert!(parser.tokens.next().is_none());
        }

        #[test]
        fn positive_float_with_unit() {
            let mut parser = Parser::new("3.14rem".chars());
            let result = parser.parse::<Length>().unwrap();

            assert_eq!(result, Length::Length(3.14, LengthUnit::RootFontSize));
            assert!(parser.tokens.next().is_none());
        }

        #[test]
        fn negative_float_with_unit() {
            let mut parser = Parser::new("-1000000rem".chars());
            let result = parser.parse::<Length>().unwrap();

            assert_eq!(result, Length::Length(-1000000.0, LengthUnit::RootFontSize));
            assert!(parser.tokens.next().is_none());
        }
    }

    mod double {
        use crate::parser::percentage::Percentage;

        use super::*;

        #[test]
        fn nothing() {
            assert!(parse_side_length("").is_err());
        }

        #[test]
        fn two_zeros() {
            let result = parse_side_length("0 0");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Length(Length::Zero()),
                    LengthOrPercentage::Length(Length::Zero())
                ))
            )
        }

        #[test]
        fn zero_and_percentage() {
            let result = parse_side_length("0 5%");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Length(Length::Zero()),
                    LengthOrPercentage::Percentage(Percentage(5.0))
                ))
            );
        }

        #[test]
        fn percentage_and_zero() {
            let result = parse_side_length("25% 0");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Percentage(Percentage(25.0)),
                    LengthOrPercentage::Length(Length::Zero())
                ))
            );
        }

        #[test]
        fn two_lengths() {
            let result = parse_side_length("21rem -78px");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Length(Length::Length(21.0, LengthUnit::RootFontSize)),
                    LengthOrPercentage::Length(Length::Length(-78.0, LengthUnit::Pixels))
                ))
            )
        }

        #[test]
        fn two_percentages() {
            let result = parse_side_length("24% 70%");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Percentage(Percentage(24.0)),
                    LengthOrPercentage::Percentage(Percentage(70.0))
                ))
            );
        }

        #[test]
        fn zero_and_length() {
            let result = parse_side_length("0 284px");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Length(Length::Zero()),
                    LengthOrPercentage::Length(Length::Length(284.0, LengthUnit::Pixels))
                ))
            );
        }

        #[test]
        fn length_and_zero() {
            let result = parse_side_length("12vmax 0");

            assert_eq!(
                result,
                Ok(Sides::Double(
                    LengthOrPercentage::Length(Length::Length(12.0, LengthUnit::ViewportMaximum)),
                    LengthOrPercentage::Length(Length::Zero())
                ))
            );
        }
    }

    mod quad {
        use crate::parser::percentage::Percentage;

        use super::*;

        #[test]
        fn four_units() {
            let result = parse_side_length("12px 24px 36rem 48rem");

            assert_eq!(
                result,
                Ok(Sides::Quad(
                    LengthOrPercentage::Length(Length::Length(12.0, LengthUnit::Pixels)),
                    LengthOrPercentage::Length(Length::Length(24.0, LengthUnit::Pixels)),
                    LengthOrPercentage::Length(Length::Length(36.0, LengthUnit::RootFontSize)),
                    LengthOrPercentage::Length(Length::Length(48.0, LengthUnit::RootFontSize))
                ))
            );
        }

        #[test]
        fn two_units_two_percentages() {
            let result = parse_side_length("45pt 50% 100pt 0%");

            assert_eq!(
                result,
                Ok(Sides::Quad(
                    LengthOrPercentage::Length(Length::Length(45.0, LengthUnit::Points)),
                    LengthOrPercentage::Percentage(Percentage(50.0)),
                    LengthOrPercentage::Length(Length::Length(100.0, LengthUnit::Points)),
                    LengthOrPercentage::Percentage(Percentage(0.0))
                ))
            )
        }

        #[test]
        fn three_units_one_percentage() {
            let result = parse_side_length("30px -25px 50% -100px");

            assert_eq!(
                result,
                Ok(Sides::Quad(
                    LengthOrPercentage::Length(Length::Length(30.0, LengthUnit::Pixels)),
                    LengthOrPercentage::Length(Length::Length(-25.0, LengthUnit::Pixels)),
                    LengthOrPercentage::Percentage(Percentage(50.0)),
                    LengthOrPercentage::Length(Length::Length(-100.0, LengthUnit::Pixels))
                ))
            )
        }

        #[test]
        fn three_percentages_one_unit() {
            let result = parse_side_length("23% -18.2% 65.4rem 29%");

            assert_eq!(
                result,
                Ok(Sides::Quad(
                    LengthOrPercentage::Percentage(Percentage(23.0)),
                    LengthOrPercentage::Percentage(Percentage(-18.2)),
                    LengthOrPercentage::Length(Length::Length(65.4, LengthUnit::RootFontSize)),
                    LengthOrPercentage::Percentage(Percentage(29.0))
                ))
            )
        }
    }
}
