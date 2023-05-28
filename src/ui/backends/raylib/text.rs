use std::collections::HashMap;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::measure_text_ex;
use crate::ui::backends::raylib::raylib::spacing_unit_to_pixels;
use crate::ui::frontend::{ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar};

pub fn draw_text(t: TextElement,window_width: i32,window_height: i32,d: &mut RaylibDrawHandle,data_hashmap: &HashMap<String,String>) {
    let color = t.styles.color.unwrap_or(RGBColor {
        red: 255,
        green: 255,
        blue: 255
    });

    let left_margin = spacing_unit_to_pixels(t.styles.margin_left.unwrap_or(SpacingUnit::Pixels(0)),window_width,window_height);
    let right_margin = spacing_unit_to_pixels(t.styles.margin_right.unwrap_or(SpacingUnit::Pixels(0)),window_width,window_height);
    let top_margin = spacing_unit_to_pixels(t.styles.margin_top.unwrap_or(SpacingUnit::Pixels(0)),window_width,window_height);
    let bottom_margin = spacing_unit_to_pixels(t.styles.margin_bottom.unwrap_or(SpacingUnit::Pixels(0)),window_width,window_height);

    let font_size = spacing_unit_to_pixels(t.styles.font_size.unwrap_or(SpacingUnit::Pixels(16)),window_width,window_height);

    let value = match t.content {
        ValueOrVar::Value(value) => {
            value
        },
        ValueOrVar::Variable(var) => {
            data_hashmap.get(&var).unwrap().to_string()
        }
    };

    match t.styles.alignment.unwrap_or(ElementAlignment::TopLeft) {
        ElementAlignment::TopLeft => {
            d.draw_text(&value,0 + left_margin - right_margin,0 + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::TopRight => {
            d.draw_text(&value,window_width + left_margin - right_margin,0 + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::BottomRight => {
            d.draw_text(&value,window_width + left_margin - right_margin,window_height + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::BottomLeft => {
            d.draw_text(&value,0 + left_margin - right_margin,window_height + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::CenterHorizontal => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,((window_height / 2) - text_size.x as i32) + left_margin - right_margin,0 + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::CenterVertical => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,0 + left_margin - right_margin,((window_height / 2) - text_size.y as i32) + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
        ElementAlignment::CenterVerticalAndHorizontal => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,((window_height / 2) - text_size.x as i32) + left_margin - right_margin,((window_height / 2) - text_size.y as i32) + top_margin - bottom_margin,font_size,Color {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: 1,
            })
        },
    }
}