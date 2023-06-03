use crate::ui::backends::raylib::raylib::spacing_unit_to_pixels;
use crate::ui::frontend::{
    ButtonElement, ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar,
};
use crate::FlameEngineView;
use hashbrown::HashMap;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON;
use raylib::ffi::{CheckCollisionPointRec, GetMousePosition, Rectangle};
use raylib::text::measure_text_ex;

pub(crate) fn draw_button(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    bg_color: Color,
    color: Color,
    font_size: i32,
    text: &str,
    d: &mut RaylibDrawHandle,
) -> bool {
    d.draw_rectangle(x, y, width, height, bg_color);

    let text_size = measure_text_ex(d.get_font_default(), &text, font_size as f32, 0 as f32);
    d.draw_text(
        text,
        x + ((width / 2) as f32 - text_size.x / 2.0) as i32,
        y + ((height / 2) as f32 - text_size.y / 2.0) as i32,
        font_size,
        color,
    );

    let btn_bounds = Rectangle {
        x: x as f32,
        y: y as f32,
        width: width as f32,
        height: height as f32,
    };

    // Check button state
    unsafe {
        let mouse_point = GetMousePosition();
        if CheckCollisionPointRec(mouse_point, btn_bounds) {
            if d.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub fn rgb_color_to_raylib_color(color: RGBColor) -> Color {
    Color {
        r: color.red,
        g: color.green,
        b: color.blue,
        a: 250,
    }
}

pub fn draw_button_handler(
    b: &ButtonElement,
    window_width: i32,
    window_height: i32,
    d: &mut RaylibDrawHandle,
    data_hashmap: &HashMap<String, String>,
    function_map: &HashMap<String, fn(&mut FlameEngineView)>,
    flame_view: &mut FlameEngineView,
) {
    let left_margin =
        spacing_unit_to_pixels(b.styles.margin_left.clone(), window_width, window_height);
    let right_margin =
        spacing_unit_to_pixels(b.styles.margin_right.clone(), window_width, window_height);
    let top_margin =
        spacing_unit_to_pixels(b.styles.margin_top.clone(), window_width, window_height);
    let bottom_margin =
        spacing_unit_to_pixels(b.styles.margin_bottom.clone(), window_width, window_height);

    let width = spacing_unit_to_pixels(b.styles.width.clone(), window_width, window_height);
    let height = spacing_unit_to_pixels(b.styles.height.clone(), window_width, window_height);

    let font_size = 18;

    let value = match &b.content {
        ValueOrVar::Value(value) => value.to_string(),
        ValueOrVar::Variable(var) => {
            format!("{}{}", var.1, data_hashmap.get(&var.0).unwrap())
        }
    };

    let mut button_pressed ;

    match b.styles.alignment {
        ElementAlignment::TopLeft => {
            button_pressed = draw_button(
                0 + left_margin - right_margin,
                0 + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::TopRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                0 + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::BottomRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                window_height + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::BottomLeft => {
            button_pressed = draw_button(
                0 + left_margin - right_margin,
                window_height + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::CenterHorizontal => {
            button_pressed = draw_button(
                ((window_width / 2) - width / 2) + left_margin - right_margin,
                0 + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::CenterVertical => {
            button_pressed = draw_button(
                0 + left_margin - right_margin,
                ((window_height / 2) - height) + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
        ElementAlignment::CenterVerticalAndHorizontal => {
            button_pressed = draw_button(
                ((window_width / 2) - width / 2) + left_margin - right_margin,
                ((window_height / 2) - height) + top_margin - bottom_margin,
                width,
                height,
                rgb_color_to_raylib_color(b.styles.background_color.clone()),
                rgb_color_to_raylib_color(b.styles.color.clone()),
                font_size,
                &value,
                d,
            );
        }
    }

    if button_pressed {
        let action = function_map.get(&b.binding).unwrap();
        action(flame_view);
    }
}
