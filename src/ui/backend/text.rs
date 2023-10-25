use glyphon::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, TextArea, TextBounds};
use glyphon::cosmic_text::Align;
use crate::ui::backend::helpers::{measure_text, spacing_unit_to_pixels};
use crate::ui::frontend::{ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar};
use hashbrown::HashMap;
use log::warn;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{GlyphBrush, Layout, Section, Text};
use crate::renderer::CustomTextArea;

pub fn draw_text<'a>(
    t: &TextElement,
    window_width: f32,
    window_height: f32,
    data_hashmap: &HashMap<String, String>,
    font_system: &mut FontSystem
) -> Vec<CustomTextArea> {
    let mut out: Vec<CustomTextArea> = Vec::new();
    let left_margin =
        spacing_unit_to_pixels(t.styles.margin_left.clone(), window_width, window_height) as f32;
    let right_margin =
        spacing_unit_to_pixels(t.styles.margin_right.clone(), window_width, window_height) as f32;
    let top_margin =
        spacing_unit_to_pixels(t.styles.margin_top.clone(), window_width, window_height) as f32;
    let bottom_margin =
        spacing_unit_to_pixels(t.styles.margin_bottom.clone(), window_width, window_height) as f32;

    let font_size =
        spacing_unit_to_pixels(t.styles.font_size.clone(), window_width, window_height) as f32;

    let value = match &t.content {
        ValueOrVar::Value(value) => value.to_string(),
        ValueOrVar::Variable(var) => {
            format!("{}{}", var.1, data_hashmap.get(&var.0).unwrap())
        }
    };


    match t.styles.alignment {
        ElementAlignment::TopLeft => {

            let mut buffer = Buffer::new(font_system, Metrics::new(font_size, 42.0));
            buffer.set_size(font_system, window_width, window_height);
            buffer.set_text(font_system, &value, Attrs::new().family(Family::SansSerif), Shaping::Advanced);


            out.push(CustomTextArea {
                buffer: buffer,
                left: left_margin - right_margin,
                top:  top_margin - bottom_margin,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: window_width as i32,
                    bottom: window_height as i32,
                },
                default_color: Color::rgb(t.styles.color.red, t.styles.color.green,  t.styles.color.blue),
            });
        }
        ElementAlignment::TopRight => {
            println!("TOP RIGHT {}",window_width);

            let mut buffer = Buffer::new(font_system, Metrics::new(font_size, 42.0));
            buffer.set_size(font_system, window_width, window_height);
            buffer.set_text(font_system, &value, Attrs::new().family(Family::SansSerif), Shaping::Advanced);
            // for line in &mut buffer.lines {
            //     line.set_align(Some(Align::Left));
            // }


            out.push(CustomTextArea {
                buffer: buffer,
                left: 10.0,
                top:  10.0,
                scale: 1.0,
                bounds: TextBounds {
                    left: window_width as i32,
                    top: window_height as i32,
                    right: window_width as i32,
                    bottom: window_height as i32,
                },
                default_color: Color::rgb(t.styles.color.red, t.styles.color.green,  t.styles.color.blue),
            });
        }
        ElementAlignment::BottomRight => {
            // glyph.queue(Section {
            //     screen_position: (
            //         window_width + left_margin - right_margin,
            //         window_height + top_margin - bottom_margin,
            //     ),
            //     bounds: (window_width as f32, window_height as f32),
            //     text: vec![Text::new(&value)
            //         .with_color([
            //             t.styles.color.red as f32 / 255.0,
            //             t.styles.color.green as f32 / 255.0,
            //             t.styles.color.blue as f32 / 255.0,
            //             1.0,
            //         ])
            //         .with_scale(font_size as f32 * 5.0)],
            //     ..Section::default()
            // });
        }
        ElementAlignment::BottomLeft => {
            // glyph.queue(Section {
            //     screen_position: (
            //         0.0 + left_margin - right_margin,
            //         window_height + top_margin - bottom_margin,
            //     ),
            //     bounds: (window_width as f32, window_height as f32),
            //     text: vec![Text::new(&value)
            //         .with_color([
            //             t.styles.color.red as f32 / 255.0,
            //             t.styles.color.green as f32 / 255.0,
            //             t.styles.color.blue as f32 / 255.0,
            //             1.0,
            //         ])
            //         .with_scale(font_size as f32 * 5.0)],
            //     ..Section::default()
            // });
        }
        ElementAlignment::CenterHorizontal => {
            // let text_size = measure_text(font, &value, font_size * 5.0);
            // let x = ((window_width / 2.0) - text_size.0) + left_margin - right_margin;
            // glyph.queue(Section {
            //     screen_position: (
            //         0.0 + left_margin - right_margin,
            //         0.0 + top_margin - bottom_margin,
            //     ),
            //     bounds: (window_width as f32, window_height as f32),
            //     text: vec![Text::new(&value)
            //         .with_color([
            //             t.styles.color.red as f32 / 255.0,
            //             t.styles.color.green as f32 / 255.0,
            //             t.styles.color.blue as f32 / 255.0,
            //             1.0,
            //         ])
            //         .with_scale(font_size as f32 * 5.0)],
            //     ..Section::default()
            // });
        }
        ElementAlignment::CenterVertical => {
            // let text_size = measure_text(font, &value, font_size * 5.0);
            // let y = ((window_height / 2.0) - text_size.1) + top_margin - bottom_margin;
            // glyph.queue(Section {
            //     screen_position: (0.0 + left_margin - right_margin, y),
            //     bounds: (window_width as f32, window_height as f32),
            //     text: vec![Text::new(&value)
            //         .with_color([
            //             t.styles.color.red as f32 / 255.0,
            //             t.styles.color.green as f32 / 255.0,
            //             t.styles.color.blue as f32 / 255.0,
            //             1.0,
            //         ])
            //         .with_scale(font_size as f32 * 5.0)],
            //     ..Section::default()
            // });
        }
        ElementAlignment::CenterVerticalAndHorizontal => {
            // let text_size = measure_text(font, &value, font_size * 5.0);
            // let y = ((window_height / 2.0) - text_size.1) + top_margin - bottom_margin;
            // let x = ((window_width / 2.0) - text_size.0) + left_margin - right_margin;
            // glyph.queue(Section {
            //     screen_position: (x, y),
            //     bounds: (window_width as f32, window_height as f32),
            //     text: vec![Text::new(&value)
            //         .with_color([
            //             t.styles.color.red as f32 / 255.0,
            //             t.styles.color.green as f32 / 255.0,
            //             t.styles.color.blue as f32 / 255.0,
            //             1.0,
            //         ])
            //         .with_scale(font_size as f32 * 5.0)],
            //
            //     ..Section::default()
            // });
        }
    }
    return out
}
