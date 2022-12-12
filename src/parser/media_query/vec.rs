use super::*;

impl CommaSeparated for MediaQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let mut parser = Parser::new("screen".chars());
        assert_eq!(
            Ok(vec![MediaQuery::MediaType(MediaType::Screen)]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn tow() {
        let mut parser = Parser::new("screen, print".chars());
        assert_eq!(
            Ok(vec![
                MediaQuery::MediaType(MediaType::Screen),
                MediaQuery::MediaType(MediaType::Print)
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn empty() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<Vec<MediaQuery>>().is_err());
    }

    #[test]
    fn no_whitespace() {
        let mut parser = Parser::new("screen,print".chars());
        assert_eq!(
            Ok(vec![
                MediaQuery::MediaType(MediaType::Screen),
                MediaQuery::MediaType(MediaType::Print)
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn extra_whitespace() {
        let mut parser = Parser::new("screen   ,   print   ".chars());
        assert_eq!(
            Ok(vec![
                MediaQuery::MediaType(MediaType::Screen),
                MediaQuery::MediaType(MediaType::Print)
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }

    #[test]
    fn trailing_comma() {
        let mut parser = Parser::new("screen,print,".chars());
        assert!(parser.parse::<Vec<MediaQuery>>().is_err());
    }

    #[test]
    fn complex() {
        let mut parser = Parser::new(
            "(color) and screen, print and (orientation: landscape), not screen, (all or not all)"
                .chars(),
        );
        assert_eq!(
            Ok(vec![
                MediaQuery::And(
                    Box::new(MediaQuery::MediaFeature(MediaFeature::Color)),
                    Box::new(MediaQuery::MediaType(MediaType::Screen)),
                ),
                MediaQuery::And(
                    Box::new(MediaQuery::MediaType(MediaType::Print)),
                    Box::new(MediaQuery::MediaFeature(MediaFeature::Orientation(
                        Orientation::Landscape
                    )))
                ),
                MediaQuery::Not(Box::new(MediaQuery::MediaType(MediaType::Screen))),
                MediaQuery::Or(
                    Box::new(MediaQuery::MediaType(MediaType::All)),
                    Box::new(MediaQuery::Not(Box::new(MediaQuery::MediaType(
                        MediaType::All
                    ))))
                )
            ]),
            parser.parse()
        );
        assert_eq!(None, parser.tokens.next());
    }
}
