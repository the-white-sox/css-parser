use super::*;

fn parse_distance(input: &str) -> Result<Distance, ParsingError> {
    let mut parser = Parser::new(input.chars());
    return parser.parse::<Distance>();
}

mod units {
    use super::*;

    fn parse_unit(input: &str) -> Result<DistanceUnit, ()> {
        return input.parse::<DistanceUnit>();
    }

    #[test]
    fn nothing() {
        assert!(parse_unit("").is_err());
    }

    #[test]
    fn pixels() {
        assert_eq!(parse_unit("px"), Ok(DistanceUnit::Pixels));
    }

    #[test]
    fn centimeters() {
        assert_eq!(parse_unit("cm"), Ok(DistanceUnit::Centimeters));
    }

    #[test]
    fn inches() {
        assert_eq!(parse_unit("in"), Ok(DistanceUnit::Inches));
    }

    #[test]
    fn points() {
        assert_eq!(parse_unit("pt"), Ok(DistanceUnit::Points));
    }

    #[test]
    fn font_size() {
        assert_eq!(parse_unit("em"), Ok(DistanceUnit::FontSize));
    }

    #[test]
    fn root_font_size() {
        assert_eq!(parse_unit("rem"), Ok(DistanceUnit::RootFontSize));
    }

    #[test]
    fn viewport_height() {
        assert_eq!(parse_unit("vh"), Ok(DistanceUnit::ViewportHeight));
    }

    #[test]
    fn viewport_width() {
        assert_eq!(parse_unit("vw"), Ok(DistanceUnit::ViewportWidth));
    }

    #[test]
    fn root_block_size() {
        assert_eq!(parse_unit("vb"), Ok(DistanceUnit::ViewportBlockSize));
    }

    #[test]
    fn root_inline_size() {
        assert_eq!(parse_unit("vi"), Ok(DistanceUnit::ViewportInlineSize));
    }

    #[test]
    fn viewport_minimum() {
        assert_eq!(parse_unit("vmin"), Ok(DistanceUnit::ViewportMinimum));
    }

    #[test]
    fn viewport_maximum() {
        assert_eq!(parse_unit("vmax"), Ok(DistanceUnit::ViewportMaximum));
    }

    #[test]
    fn invalid_unit() {
        let input = "xd";

        assert!(DistanceUnit::from_str(input).is_err());
    }
}

mod distances {
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
