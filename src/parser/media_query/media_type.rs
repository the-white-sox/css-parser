use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    All,
    Print,
    Screen,
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(MediaType::All),
            "print" => Ok(MediaType::Print),
            "screen" => Ok(MediaType::Screen),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for MediaType {
    const EXPECTED: &'static str = "all, print, or screen";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let mut parser = Parser::new("all".chars());
        assert_eq!(MediaType::All, parser.parse().unwrap());
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn print() {
        let mut parser = Parser::new("print".chars());
        assert_eq!(MediaType::Print, parser.parse().unwrap());
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn screen() {
        let mut parser = Parser::new("screen".chars());
        assert_eq!(MediaType::Screen, parser.parse().unwrap());
        assert!(parser.tokens.next().is_none());
    }

    #[test]
    fn not_media_type() {
        let mut parser = Parser::new("not a media type".chars());
        assert!(parser.parse::<MediaType>().is_err());
    }

    #[test]
    fn nothing() {
        let mut parser = Parser::new("".chars());
        assert!(parser.parse::<MediaType>().is_err());
    }
}
