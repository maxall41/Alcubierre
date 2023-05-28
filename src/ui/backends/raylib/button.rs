use hashbrown::HashMap;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::{CheckCollisionPointRec, GetMousePosition, Rectangle};
use raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON;
use raylib::text::measure_text_ex;
use crate::ui::backends::raylib::raylib::spacing_unit_to_pixels;
use crate::ui::frontend::{ButtonElement, ElementAlignment, SpacingUnit, TextElement, ValueOrVar};

pub(crate) fn draw_button(x: i32, y: i32, width: i32, height: i32,color: Color,font_size: i32, text: &str, d: &mut RaylibDrawHandle) -> bool {
    d.draw_rectangle(x,y,width,height,color);

    let text_size = measure_text_ex(d.get_font_default(), &text, font_size as f32, 0 as f32);
    d.draw_text(text,x + ((width / 2) as f32 - text_size.x) as i32,y + ((height / 2) as f32 - text_size.y) as i32,font_size,color);

    let btn_bounds = Rectangle {
        x: (x - width) as f32,
        y: (y - height) as f32,
        width: width as f32,
        height: height as f32,
    };

    // Check button state
    unsafe {
        let mouse_point = GetMousePosition();
        if CheckCollisionPointRec(mouse_point, btn_bounds)
        {
            if d.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

}

pub fn draw_button_handler(b: &ButtonElement,window_width: i32,window_height: i32,d: &mut RaylibDrawHandle,data_hashmap: &HashMap<String,String>,function_map: &HashMap<String,fn()>) {

    let left_margin = spacing_unit_to_pixels(b.styles.margin_left.clone(),window_width,window_height);
    let right_margin = spacing_unit_to_pixels(b.styles.margin_right.clone(),window_width,window_height);
    let top_margin = spacing_unit_to_pixels(b.styles.margin_top.clone(),window_width,window_height);
    let bottom_margin = spacing_unit_to_pixels(b.styles.margin_bottom.clone(),window_width,window_height);

    let width = spacing_unit_to_pixels(b.styles.width.clone(),window_width,window_height);
    let height = spacing_unit_to_pixels(b.styles.height.clone(),window_width,window_height);

    let font_size = 12;


    let value = match &b.content {
        ValueOrVar::Value(value) => {
            value
        },
        ValueOrVar::Variable(var) => {
            data_hashmap.get(var).unwrap()
        }
    };

    let mut button_pressed = false;


    match b.styles.alignment {
        ElementAlignment::TopLeft => {
            button_pressed = draw_button(0 + left_margin - right_margin, 0 + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::TopRight => {
            button_pressed = draw_button(window_width + left_margin - right_margin, 0 + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::BottomRight => {
            button_pressed = draw_button(window_width + left_margin - right_margin, window_height + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::BottomLeft => {
            button_pressed = draw_button(0 + left_margin - right_margin, window_height + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::CenterHorizontal => {
            button_pressed = draw_button(((window_width / 2) - width) + left_margin - right_margin, 0 + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::CenterVertical => {
            button_pressed = draw_button(0 + left_margin - right_margin, ((window_height / 2) - height) + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
        ElementAlignment::CenterVerticalAndHorizontal => {
            button_pressed = draw_button(((window_width / 2) - width) + left_margin - right_margin, ((window_height / 2) - height) + top_margin - bottom_margin, width, height, Color {
                r: b.styles.color.red,
                g: b.styles.color.green,
                b: b.styles.color.blue,
                a: 100,
            }, font_size, &value, d);
        },
    }

    if button_pressed {
        let action = function_map.get(&b.binding).unwrap();
        action();
    }
}