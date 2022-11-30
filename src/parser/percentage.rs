#[ctg(test)]
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

        assert_eq!(result, Percentage(0.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_int() {
        let mut parser = Parser::new("23%".chars());
        let result = parser.parse::<Distance>().unwrap();

        assert_eq!(result, Percentage(23.0));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn positive_float() {
        let mut parser = Parser::new("62.3%".chars());
        let result = parser.parse::<Percentage>().unwrap();

        assert_eq!(result, Percentage(62.3));
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn negative_int() {
        assert!(parse_distance("-87%").is_err());
    }
}
