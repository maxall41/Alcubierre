use crate::ui::frontend::SpacingUnit;
use wgpu_glyph::ab_glyph::{Font, FontArc};

pub(crate) fn spacing_unit_to_pixels(
    unit: SpacingUnit,
    window_width: f32,
    window_height: f32,
) -> i32 {
    match unit {
        SpacingUnit::Pixels(p) => p,
        SpacingUnit::PercentHeight(percent) => {
            let percent_mult = percent as f32 / 100.0;
            (window_height * percent_mult) as i32
        }
        SpacingUnit::PercentWidth(percent) => {
            let percent_mult = percent as f32 / 100.0;
            (window_width * percent_mult) as i32
        }
    }
}

pub(crate) fn measure_text(font: &FontArc, text: &str, font_size: f32) -> (f32, f32) {
    let mut text_width = 0.0;
    let mut glyph_heights = 0.0;
    let glyph_count = text.len();

    for glyph in 0..glyph_count {
        //TODO: I don't think this really handles new line characters
        let char = text.chars().nth(glyph).unwrap();
        let glyph = font.glyph_id(char).with_scale(font_size);
        let rect = font.glyph_bounds(&glyph);
        text_width += rect.width();
        glyph_heights += rect.height();
    }

    let average_text_height = glyph_heights / glyph_count as f32;

    (text_width / 2.2, average_text_height / 2.2)
}
