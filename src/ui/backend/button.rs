use crate::ui::frontend::{
    ButtonElement, ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar,
};
use hashbrown::HashMap;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{GlyphBrush, Section, Text};
use crate::game_object::behaviours::EngineView;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::renderer::camera::CameraUniform;
use crate::ui::backend::helpers::{measure_text, spacing_unit_to_pixels};

pub(crate) fn draw_button(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    bg_color: RGBColor,
    color: RGBColor,
    font_size: f32,
    text: &str,
    font: &FontArc,
    buffer: &mut QuadBufferBuilder,
    glyph: &mut GlyphBrush<()>,
    uniform: &CameraUniform,
    raw_window_pos: (f32,f32),
) -> bool {
    // d.draw_rectangle(x, y, width, height, bg_color);
    // println!("{:?}",uniform.view_proj);
    // println!("{:?},{:?}",uniform.view_proj[0][0],uniform.view_proj[1][1]);
    let rect_x = x / 100.0;
    let rect_y = y / 100.0;

    // println!("RECT X: {:?}, Y: {:?}, W: {}, H: {}",rect_x,rect_y,width,height);

    buffer.push_rect(rect_x,rect_y,width / 10.0,height / 10.0,&bg_color);

    let text_size = measure_text(font,&text,font_size * 2.0);

    let text_y = y + ((height / 2.0) as f32 - text_size.1 * 2.2);
    let text_x = x + ((width / 2.0) - text_size.0);

    println!("{},{}",text_x,text_y);

    glyph.queue(Section {
        screen_position: (text_x,text_y),
        bounds: (1000.0,1000.0),
        text: vec![Text::new(&text)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(font_size as f32 * 2.0)],
        ..Section::default()
    });

    // let text_size = measure_text_ex(d.get_font_default(), &text, font_size as f32, 0 as f32);
    // d.draw_text(
    //     text,
    //     x + ((width / 2) as f32 - text_size.x / 2.0) as i32,
    //     y + ((height / 2) as f32 - text_size.y / 2.0) as i32,
    //     font_size,
    //     color,
    // );

    // let btn_bounds = Rectangle {
    //     x: x as f32,
    //     y: y as f32,
    //     width: width as f32,
    //     height: height as f32,
    // };
    //
    // // Check button state
    // unsafe {
    //     let mouse_point = GetMousePosition();
    //     if CheckCollisionPointRec(mouse_point, btn_bounds) {
    //         if d.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
    //             true
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    // }

    false
}

pub fn draw_button_handler(
    b: &ButtonElement,
    window_width: f32,
    window_height: f32,
    glyph: &mut GlyphBrush<()>,
    font: &FontArc,
    data_hashmap: &HashMap<String, String>,
    function_map: &HashMap<String, fn(&mut EngineView)>,
    buffer: &mut QuadBufferBuilder,
    view: &mut EngineView,
    uniform: &CameraUniform
) {
    let left_margin =
        spacing_unit_to_pixels(b.styles.margin_left.clone(), window_width, window_height) as f32;
    let right_margin =
        spacing_unit_to_pixels(b.styles.margin_right.clone(), window_width, window_height) as f32;
    let top_margin =
        spacing_unit_to_pixels(b.styles.margin_top.clone(), window_width, window_height) as f32;
    let bottom_margin =
        spacing_unit_to_pixels(b.styles.margin_bottom.clone(), window_width, window_height) as f32;

    let width = spacing_unit_to_pixels(b.styles.width.clone(), window_width, window_height) as f32;
    let height = spacing_unit_to_pixels(b.styles.height.clone(), window_width, window_height) as f32;

    let font_size = 18.0;

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
                0.0 + left_margin - right_margin,
                0.0 - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform,
                (((window_width / 2.0) - width / 2.0) + left_margin - right_margin,)
            );
        }
        ElementAlignment::TopRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                0.0 - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
        ElementAlignment::BottomRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                window_height - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
        ElementAlignment::BottomLeft => {
            button_pressed = draw_button(
                0.0 + left_margin - right_margin,
                window_height - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
        ElementAlignment::CenterHorizontal => {
            button_pressed = draw_button(
                0.0 + left_margin - right_margin,
                0.0 - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
        ElementAlignment::CenterVertical => {
            button_pressed = draw_button(
                0.0,
                0.0 - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
        ElementAlignment::CenterVerticalAndHorizontal => {
            button_pressed = draw_button(
                0.0 + left_margin - right_margin,
                0.0 - top_margin + bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                uniform
            );
        }
    }

    if button_pressed {
        let action = function_map.get(&b.binding).unwrap();
        action(view);
    }
}
