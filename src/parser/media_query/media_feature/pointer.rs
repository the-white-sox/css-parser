use super::*;

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
