use crate::parser::length::Distance;

use super::*;

mod color_scheme;
mod hover;
mod orientation;
mod pointer;

use color_scheme::*;
use hover::*;
use orientation::*;
use pointer::*;

pub enum MediaFeature {
    Color,
    MonoChrome,
    MinWidth(Distance),
    Width(Distance),
    MaxWidth(Distance),
    MinHeight(Distance),
    Height(Distance),
    MaxHeight(Distance),
    Orientation(Orientation),
    Hover(Hover),
    AnyHover(Hover),
    Pointer(Pointer),
    AnyPointer(Pointer),
    PrefersColorScheme(ColorScheme),
}
