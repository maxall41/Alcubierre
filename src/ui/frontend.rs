use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub(crate) mod errors;
pub(crate) mod helpers;
pub mod rough;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ValueOrVar {
    Value(String),
    Variable(String),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ElementAlignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    CenterVertical,
    CenterHorizontal,
    CenterVerticalAndHorizontal,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SpacingUnit {
    Pixels(i32),
    PercentWidth(i32),
    PercentHeight(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TextStyleData {
    pub font: Option<String>,
    pub font_size: Option<SpacingUnit>,
    pub alignment: Option<ElementAlignment>,
    pub color: Option<RGBColor>,
    pub margin_top: Option<SpacingUnit>,
    pub margin_bottom: Option<SpacingUnit>,
    pub margin_left: Option<SpacingUnit>,
    pub margin_right: Option<SpacingUnit>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ButtonStyleData {
    pub font: Option<String>,
    pub font_size_px: Option<SpacingUnit>,
    pub alignment: Option<ElementAlignment>,
    pub color: Option<RGBColor>,
    pub background_color: Option<RGBColor>,
    pub margin_top: Option<SpacingUnit>,
    pub margin_bottom: Option<SpacingUnit>,
    pub margin_left: Option<SpacingUnit>,
    pub margin_right: Option<SpacingUnit>,
    pub width:  Option<SpacingUnit>,
    pub height: Option<SpacingUnit>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TextElement {
    pub content: ValueOrVar,
    pub styles: TextStyleData,
    pub classes: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ButtonElement {
    pub content: ValueOrVar,
    pub styles: ButtonStyleData,
    pub classes: Vec<String>,
    pub binding: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Element {
    Text(TextElement),
    Button(ButtonElement),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HyperFoilAST {
    pub elements: Vec<Element>,
}
