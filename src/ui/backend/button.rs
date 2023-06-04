use crate::game_object::behaviours::EngineView;
use crate::game_object::graphics::RectData;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::renderer::camera::{screen_space_to_view_space, Camera, Projection};
use crate::ui::backend::helpers::{measure_text, spacing_unit_to_pixels};
use crate::ui::frontend::{
    ButtonElement, ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar,
};
use crate::MouseData;
use hashbrown::HashMap;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{GlyphBrush, Section, Text};
use winit::dpi::PhysicalSize;

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
    window_width: f32,
    window_height: f32,
    projection: &Projection,
    mouse_data: &MouseData,
) -> bool {
    // d.draw_rectangle(x, y, width, height, bg_color);
    // println!("RS: {},{}",x,y);

    let rect_space = screen_space_to_view_space(
        x as u32,
        y as u32,
        window_width,
        window_height,
        projection.calc_matrix(),
    );

    buffer.push_rect(
        rect_space.0,
        rect_space.1,
        width / 10.0,
        height / 10.0,
        &bg_color,
    );

    let text_size = measure_text(font, &text, font_size * 2.0);

    // Magic Numbers. Do not touch
    let text_x = x + ((width / 2.0) as f32 - text_size.0 * 1.4);
    let text_y = y + ((height / 2.0) as f32 - text_size.1 * 1.6);

    glyph.queue(Section {
        screen_position: (text_x, text_y),
        bounds: (1000.0, 1000.0),
        text: vec![Text::new(&text)
            .with_color([
                color.red as f32 / 255.0,
                color.green as f32 / 255.0,
                color.blue as f32 / 255.0,
                1.0,
            ])
            .with_scale(font_size as f32 * 2.0)],
        ..Section::default()
    });

    // More magic numbers. Do not touch
    if mouse_data.is_left_pressed {
        if mouse_data.mouse_position.x > x as f64 / 1.4
            && mouse_data.mouse_position.x < (x + width * 5.0) as f64
        {
            if mouse_data.mouse_position.y > y as f64 / 1.05
                && mouse_data.mouse_position.y < (y + height * 2.0) as f64
            {
                return true;
            }
        }
    }

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
    projection: &Projection,
    mouse_data: &MouseData,
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
    let height =
        spacing_unit_to_pixels(b.styles.height.clone(), window_width, window_height) as f32;

    let font_size = 18.0;

    let value = match &b.content {
        ValueOrVar::Value(value) => value.to_string(),
        ValueOrVar::Variable(var) => {
            format!("{}{}", var.1, data_hashmap.get(&var.0).unwrap())
        }
    };

    let mut button_pressed;

    match b.styles.alignment {
        ElementAlignment::TopLeft => {
            button_pressed = draw_button(
                0.0 + left_margin - right_margin,
                0.0 + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::TopRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                0.0 + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::BottomRight => {
            button_pressed = draw_button(
                window_width + left_margin - right_margin,
                window_height + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::BottomLeft => {
            button_pressed = draw_button(
                0.0 + left_margin - right_margin,
                window_height + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::CenterHorizontal => {
            button_pressed = draw_button(
                ((window_width / 2.0) - width / 2.0) + left_margin - right_margin,
                0.0 + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::CenterVertical => {
            button_pressed = draw_button(
                0.0,
                ((window_height / 2.0) - height) + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
        ElementAlignment::CenterVerticalAndHorizontal => {
            button_pressed = draw_button(
                ((window_width / 2.0) - width / 2.0) + left_margin - right_margin,
                ((window_height / 2.0) - height) + top_margin - bottom_margin,
                width,
                height,
                b.styles.background_color.clone(),
                b.styles.color.clone(),
                font_size,
                &value,
                font,
                buffer,
                glyph,
                window_width,
                window_height,
                projection,
                mouse_data,
            );
        }
    }

    if button_pressed {
        let action = function_map.get(&b.binding).unwrap();
        action(view);
    }
}
