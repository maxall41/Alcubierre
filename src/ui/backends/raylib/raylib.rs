use std::collections::HashMap;
use raylib::color::Color;
use crate::ui::frontend::{Element, ElementAlignment, HyperFoilAST, RGBColor, SpacingUnit, ValueOrVar};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::text::measure_text_ex;
use crate::ui::backends::raylib::button::{draw_button, draw_button_handler};
use crate::ui::backends::raylib::text::draw_text;


pub(crate) fn spacing_unit_to_pixels(unit: SpacingUnit,window_width: i32,window_height: i32) -> i32 {
    match unit {
        SpacingUnit::Pixels(p) => p,
        SpacingUnit::PercentHeight(percent) => {
            let percent_mult = percent as f32 / 100.0;
            (window_height as f32 * percent_mult) as i32
        },
        SpacingUnit::PercentWidth(percent) => {
            let percent_mult = percent as f32 / 100.0;
            (window_width as f32 * percent_mult) as i32
        }
     }
}

pub fn process_ast_to_raylib_calls(ast: HyperFoilAST, mut d: &mut RaylibDrawHandle,window_width: i32,window_height: i32,data_hashmap: HashMap<String,String>,function_map: HashMap<String,fn()>) {
    for element in ast.elements {
        match element {
            Element::Text(t) => {
                draw_text(t,window_width,window_height,d,&data_hashmap);
            }
            Element::Button(b) => {
                draw_button_handler(b,window_width,window_height,d,&data_hashmap,&function_map);
            }
        }
    }
}
