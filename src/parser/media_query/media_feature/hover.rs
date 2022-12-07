use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Hover {
    Hover,
    None,
}

impl FromStr for Hover {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hover" => Ok(Hover::Hover),
            "none" => Ok(Hover::None),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for Hover {
    const EXPECTED: &'static str = "hover or none";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover() {
        assert_eq!(Ok(Hover::Hover), "hover".parse());
    }

    #[test]
    fn none() {
        assert_eq!(Ok(Hover::None), "none".parse());
    }

    #[test]
    fn not_hover() {
        assert!("not hover".parse::<Hover>().is_err());
    }
}
