use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ColorScheme {
    Light,
    Dark,
}

impl FromStr for ColorScheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(ColorScheme::Light),
            "dark" => Ok(ColorScheme::Dark),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for ColorScheme {
    const EXPECTED: &'static str = "light or dark";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light() {
        assert_eq!(Ok(ColorScheme::Light), "light".parse::<ColorScheme>());
    }

    #[test]
    fn dark() {
        assert_eq!(Ok(ColorScheme::Dark), "dark".parse::<ColorScheme>());
    }

    #[test]
    fn not_color_scheme() {
        assert!("not a color scheme".parse::<ColorScheme>().is_err());
    }
}
