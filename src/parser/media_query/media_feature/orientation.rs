use super::*;

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
