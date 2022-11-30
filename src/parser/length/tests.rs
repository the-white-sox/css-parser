use super::*;

fn parse_distance(input: &str) -> Result<Length, ParsingError> {
    let mut parser = Parser::new(input.chars());
    return parser.parse::<Length>();
}

mod units {
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
        let input = "xd";

        assert!(LengthUnit::from_str(input).is_err());
    }
}

mod lengths {
    use super::*;

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
