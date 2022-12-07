// <declaration-list> ::= <declaration> ";" <declaration-list> | <declaration> | <declaration> ";"
// <declaration> ::= <color-property> ":" <color> | <sides-color-property> ":" <sides-color>
// <declaration> ::= <length-property> ":" <length-or-percentage> | <side-lengths-property> ":" <side-lengths>
// <declaration> ::= "font-family" ":" <string>
// <declaration> ::= "opacity" ":" <alpha>
// <declaration> ::= "text-align" ":" <text-align-value>
// <color-property> ::= "color" | "background-color"
// <sides-color-property> ::= "border-color"
// <length-property> ::= "font-size" | "height" | "width"
// <side-lengths-property> ::= "margin" | "padding" | "border-width" | "border-radius"

use super::*;
use crate::tokenizer::*;

#[derive(Debug, PartialEq)]

pub enum declaration {
    BackgroundColor(Color),
    BorderColor(Color),
    Opacity(f64),
    FontFamily(FontFamily),
    FontSize(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
    BorderWidth(Length),
    BorderRadius(Length),
}
