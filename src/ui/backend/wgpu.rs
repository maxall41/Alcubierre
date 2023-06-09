use crate::game_object::behaviours::EngineView;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::renderer::camera::{Camera, Projection};
use crate::ui::backend::button::draw_button_handler;
use crate::ui::backend::text::draw_text;
use crate::ui::frontend::{Element, HyperFoilAST};
use crate::MouseData;
use hashbrown::HashMap;
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::GlyphBrush;
use winit::dpi::PhysicalSize;

pub fn render_from_hyperfoil_ast(
    ast: &HyperFoilAST,
    glyph_brush: &mut GlyphBrush<()>,
    size: PhysicalSize<u32>,
    font: &FontArc,
    data_map: &HashMap<String, String>,
    function_map: &HashMap<String, fn(&mut EngineView)>,
    buffer: &mut QuadBufferBuilder,
    view: &mut EngineView,
    projection: &Projection,
    mouse_data: &MouseData,
) {
    for element in &ast.elements {
        match element {
            Element::Text(t) => {
                draw_text(
                    t,
                    size.width as f32,
                    size.height as f32,
                    glyph_brush,
                    font,
                    data_map,
                );
            }
            Element::Button(b) => {
                draw_button_handler(
                    b,
                    size.width as f32,
                    size.height as f32,
                    glyph_brush,
                    font,
                    data_map,
                    function_map,
                    buffer,
                    view,
                    projection,
                    mouse_data,
                );
            }
        }
    }
}
