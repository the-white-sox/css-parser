use super::*;

fn parse_distance(input: &str) -> Result<Distance, ParsingError> {
    let mut parser = Parser::new(input.chars());
    return parser.parse::<Distance>();
}

mod units {
    use super::*;

    #[test]
    fn nothing() {
        let input = "";

        assert!(DistanceUnit::from_str(input).is_err());
    }

    #[test]
    fn pixels() {
        let input = "px";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Pixels);
    }

    #[test]
    fn centimeters() {
        let input = "cm";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Centimeters);
    }

    #[test]
    fn inches() {
        let input = "in";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Inches);
    }

    #[test]
    fn points() {
        let input = "pt";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::Points);
    }

    #[test]
    fn font_size() {
        let input = "em";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::FontSize);
    }

    #[test]
    fn root_font_size() {
        let input = "rem";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootFontSize);
    }

    #[test]
    fn viewport_height() {
        let input = "vh";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportHeight);
    }

    #[test]
    fn viewport_width() {
        let input = "vw";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportWidth);
    }

    #[test]
    fn root_block_size() {
        let input = "vb";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootBlockSize);
    }

    #[test]
    fn root_inline_size() {
        let input = "vi";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::RootInlineSize);
    }

    #[test]
    fn viewport_minimum() {
        let input = "vmin";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportMinimum);
    }

    #[test]
    fn viewport_maximum() {
        let input = "vmax";
        let unit = DistanceUnit::from_str(input).unwrap();

        assert_eq!(unit, DistanceUnit::ViewportMaximum);
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

mod percentages {
    use super::*;

    #[test]
    fn percent_no_value() {
        assert!(parse_distance("%").is_err());
    }

    #[test]
    fn zero() {
        let mut parser = Parser::new("0%".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Percentage(0.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_int() {
        let mut parser = Parser::new("23%".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Distance::Percentage(23.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_int() {
        assert!(parse_distance("-87%").is_err());
    }
}
