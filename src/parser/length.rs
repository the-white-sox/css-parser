#[derive(Debug, PartialEq)]
pub enum Unit {
    Zero(),
    Unit(f64, UnitType),
}

#[derive(Debug, PartialEq)]
pub enum UnitType {
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
