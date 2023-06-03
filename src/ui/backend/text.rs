use crate::ui::backend::helpers::{measure_text, spacing_unit_to_pixels};
use crate::ui::frontend::{ElementAlignment, RGBColor, SpacingUnit, TextElement, ValueOrVar};
use hashbrown::HashMap;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{GlyphBrush, Layout, Section, Text};

pub fn draw_text(
    t: &TextElement,
    window_width: f32,
    window_height: f32,
    glyph: &mut GlyphBrush<()>,
    font: &FontArc,
    data_hashmap: &HashMap<String, String>,
) {
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

    // I'm aware wgpu_glyph has it's own layout engine. But it doesn't seem to work for some reason. So I just wrote my own.
    match t.styles.alignment {
        ElementAlignment::TopLeft => {
            let x = (0.0 + left_margin - right_margin) as f32;
            let y = (0.0 + top_margin - bottom_margin) as f32;

            glyph.queue(Section {
                screen_position: (x, y),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::TopRight => {
            glyph.queue(Section {
                screen_position: (
                    window_width + left_margin - right_margin,
                    0.0 + top_margin - bottom_margin,
                ),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::BottomRight => {
            glyph.queue(Section {
                screen_position: (
                    window_width + left_margin - right_margin,
                    window_height + top_margin - bottom_margin,
                ),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::BottomLeft => {
            glyph.queue(Section {
                screen_position: (
                    0.0 + left_margin - right_margin,
                    window_height + top_margin - bottom_margin,
                ),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::CenterHorizontal => {
            let text_size = measure_text(font, &value, font_size * 5.0);
            let x = ((window_width / 2.0) - text_size.0) + left_margin - right_margin;
            glyph.queue(Section {
                screen_position: (
                    0.0 + left_margin - right_margin,
                    0.0 + top_margin - bottom_margin,
                ),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::CenterVertical => {
            let text_size = measure_text(font, &value, font_size * 5.0);
            let y = ((window_height / 2.0) - text_size.1) + top_margin - bottom_margin;
            glyph.queue(Section {
                screen_position: (0.0 + left_margin - right_margin, y),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],
                ..Section::default()
            });
        }
        ElementAlignment::CenterVerticalAndHorizontal => {
            let text_size = measure_text(font, &value, font_size * 5.0);
            let y = ((window_height / 2.0) - text_size.1) + top_margin - bottom_margin;
            let x = ((window_width / 2.0) - text_size.0) + left_margin - right_margin;
            glyph.queue(Section {
                screen_position: (x, y),
                bounds: (window_width as f32, window_height as f32),
                text: vec![Text::new(&value)
                    .with_color([
                        t.styles.color.red as f32 / 255.0,
                        t.styles.color.green as f32 / 255.0,
                        t.styles.color.blue as f32 / 255.0,
                        1.0,
                    ])
                    .with_scale(font_size as f32 * 5.0)],

                ..Section::default()
            });
        }
    }
}
