use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "portrait" => Ok(Orientation::Portrait),
            "landscape" => Ok(Orientation::Landscape),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for Orientation {
    const EXPECTED: &'static str = "portrait or landscape";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portrait() {
        assert_eq!(Ok(Orientation::Portrait), "portrait".parse());
    }

    #[test]
    fn landscape() {
        assert_eq!(Ok(Orientation::Landscape), "landscape".parse());
    }

    #[test]
    fn not_orientation() {
        assert!("not orientation".parse::<Orientation>().is_err());
    }
}
