use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

impl FromStr for TextAlign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(TextAlign::Left),
            "right" => Ok(TextAlign::Right),
            "center" => Ok(TextAlign::Center),
            "justify" => Ok(TextAlign::Justify),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for TextAlign {
    const EXPECTED: &'static str = "left, right, center, or justify";
}
