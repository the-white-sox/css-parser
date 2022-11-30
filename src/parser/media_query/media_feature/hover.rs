use super::*;

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
