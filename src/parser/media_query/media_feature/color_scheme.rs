use super::*;

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
