use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Pointer {
    Fine,
    Coarse,
    None,
}

impl FromStr for Pointer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fine" => Ok(Pointer::Fine),
            "coarse" => Ok(Pointer::Coarse),
            "none" => Ok(Pointer::None),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for Pointer {
    const EXPECTED: &'static str = "fine, coarse, or none";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fine() {
        assert_eq!(Ok(Pointer::Fine), "fine".parse());
    }

    #[test]
    fn coarse() {
        assert_eq!(Ok(Pointer::Coarse), "coarse".parse());
    }

    #[test]
    fn none() {
        assert_eq!(Ok(Pointer::None), "none".parse());
    }

    #[test]
    fn not_pointer() {
        assert!("not pointer".parse::<Pointer>().is_err());
    }
}
