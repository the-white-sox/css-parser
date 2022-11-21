use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Distance {
    Zero(),
    Distance(f64, DistanceUnit),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DistanceUnit {
    Pixels,          // px
    Centimeters,     // cm
    Inches,          // in
    Points,          // pt
    FontSize,        // em
    RootFontSize,    // rem
    ViewportHeight,  // vh
    ViewportWidth,   // vw
    RootBlockSize,   // vb
    RootInlineSize,  // vi
    ViewportMinimum, // vmin
    ViewportMaximum, // vmax
}

impl FromStr for DistanceUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DistanceUnit::*;

        match s {
            "px" => Ok(Pixels),
            "cm" => Ok(Centimeters),
            "in" => Ok(Inches),
            "pt" => Ok(Points),
            "em" => Ok(FontSize),
            "rem" => Ok(RootFontSize),
            "vh" => Ok(ViewportHeight),
            "vw" => Ok(ViewportWidth),
            "vb" => Ok(RootBlockSize),
            "vi" => Ok(RootInlineSize),
            "vmin" => Ok(ViewportMinimum),
            "vmax" => Ok(ViewportMaximum),
            _ => Err(()),
        }
    }
}
