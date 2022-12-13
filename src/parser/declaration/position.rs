use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "static" => Ok(Position::Static),
            "relative" => Ok(Position::Relative),
            "absolute" => Ok(Position::Absolute),
            "fixed" => Ok(Position::Fixed),
            "sticky" => Ok(Position::Sticky),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for Position {
    const EXPECTED: &'static str = "static, relative, absolute, fixed, or sticky";
}
