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

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub enum SpacingUnit {
    Pixels(i32),
    PercentWidth(i32),
    PercentHeight(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TextStyleData {
    pub font: String,
    pub font_size: SpacingUnit,
    pub alignment: ElementAlignment,
    pub color: RGBColor,
    pub margin_top: SpacingUnit,
    pub margin_bottom: SpacingUnit,
    pub margin_left: SpacingUnit,
    pub margin_right: SpacingUnit,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ButtonStyleData {
    pub font: String,
    pub font_size_px: SpacingUnit,
    pub alignment: ElementAlignment,
    pub color: RGBColor,
    pub background_color: RGBColor,
    pub margin_top: SpacingUnit,
    pub margin_bottom: SpacingUnit,
    pub margin_left: SpacingUnit,
    pub margin_right: SpacingUnit,
    pub width:  SpacingUnit,
    pub height: SpacingUnit
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
