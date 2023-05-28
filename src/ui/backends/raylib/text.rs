use hashbrown::HashMap;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::measure_text_ex;
use crate::ui::backends::raylib::raylib::spacing_unit_to_pixels;
use crate::ui::frontend::{ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar};

pub fn draw_text(t: &TextElement,window_width: i32,window_height: i32,d: &mut RaylibDrawHandle,data_hashmap: &HashMap<String,String>) {

    let left_margin = spacing_unit_to_pixels(t.styles.margin_left.clone(),window_width,window_height);
    let right_margin = spacing_unit_to_pixels(t.styles.margin_right.clone(),window_width,window_height);
    let top_margin = spacing_unit_to_pixels(t.styles.margin_top.clone(),window_width,window_height);
    let bottom_margin = spacing_unit_to_pixels(t.styles.margin_bottom.clone(),window_width,window_height);

    let font_size = spacing_unit_to_pixels(t.styles.font_size.clone(),window_width,window_height);

    let value = match &t.content {
        ValueOrVar::Value(value) => {
            value
        },
        ValueOrVar::Variable(var) => {
            data_hashmap.get(var).unwrap()
        }
    };

    match t.styles.alignment {
        ElementAlignment::TopLeft => {
            let x = 0 + left_margin - right_margin;
            let y = 0 + top_margin - bottom_margin;
            // println!("TL {},{},{:?},{}",x,y,t.styles.color,value);
            d.draw_text(&value,x,y,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::TopRight => {
            d.draw_text(&value,window_width + left_margin - right_margin,0 + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::BottomRight => {
            d.draw_text(&value,window_width + left_margin - right_margin,window_height + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::BottomLeft => {
            d.draw_text(&value,0 + left_margin - right_margin,window_height + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::CenterHorizontal => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,((window_height / 2) - text_size.x as i32) + left_margin - right_margin,0 + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::CenterVertical => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,0 + left_margin - right_margin,((window_height / 2) - text_size.y as i32) + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
        ElementAlignment::CenterVerticalAndHorizontal => {
            let text_size = measure_text_ex(d.get_font_default(), &value, font_size as f32, 0 as f32);
            d.draw_text(&value,((window_height / 2) - text_size.x as i32) + left_margin - right_margin,((window_height / 2) - text_size.y as i32) + top_margin - bottom_margin,font_size,Color {
                r: t.styles.color.red,
                g: t.styles.color.green,
                b: t.styles.color.blue,
                a: 100,
            })
        },
    }
}