use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
}

impl FromStr for Display {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "block" => Ok(Display::Block),
            "inline" => Ok(Display::Inline),
            "inline-block" => Ok(Display::InlineBlock),
            _ => Err(()),
        }
    }
}

impl FromIdentifier for Display {
    const EXPECTED: &'static str = "block, inline, or inline-block";
}
