use hashbrown::HashMap;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::GlyphBrush;
use winit::dpi::PhysicalSize;
use crate::ui::backend::text::draw_text;
use crate::ui::frontend::{Element, HyperFoilAST};

pub fn render_from_hyperfoil_ast(ast: &HyperFoilAST,glyph_brush: &mut GlyphBrush<()>,size: PhysicalSize<u32>,font: &FontArc,data_map: &HashMap<String,String>) {
    for element in &ast.elements {
        match element {
            Element::Text(t) => {
                draw_text(t, size.width as f32, size.height as f32, glyph_brush, font, data_map);
            }
            Element::Button(b) => {
                todo!()
            }
        }
    }
}