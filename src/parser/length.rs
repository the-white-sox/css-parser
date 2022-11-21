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
