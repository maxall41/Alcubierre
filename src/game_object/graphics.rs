use crate::game_object::GameObject;
use crate::renderer::atlas::{AtlasVector2, SpriteAtlas};
use crate::renderer::buffer::QuadBufferBuilder;
use crate::renderer::sprite::SpriteVertex;
use crate::ui::frontend::RGBColor;

pub type SpriteID = String;

#[derive(Clone)]
pub struct SpriteData {
    pub sprite_id: SpriteID,
    pub width: f32,
    pub height: f32,
    pub flip_h: bool,
    pub flip_v: bool
}
#[derive(Clone)]
pub struct CircleData {
    pub radius: f32,
    pub color: RGBColor,
}
#[derive(Clone)]
pub struct RectData {
    pub color: RGBColor,
    pub width: f32,
    pub height: f32,
}
#[derive(Clone)]
pub struct TriangleData {
    pub radius: f32,
    pub color: RGBColor,
}
#[derive(Clone)]
pub enum GraphicsType {
    Sprite(SpriteData),
    Circle(CircleData),
    Rect(RectData),
    Triangle(TriangleData),
}

pub trait Graphics {
    fn add_graphics(&mut self, graphics_type: GraphicsType);
    fn render(&mut self, buffer: &mut QuadBufferBuilder,sprite_verticies: &mut Vec<SpriteVertex>,
              sprite_indicies: &mut Vec<u16>,atlas: &Option<SpriteAtlas>);
}

impl Graphics for GameObject {
    fn add_graphics(&mut self, graphics_type: GraphicsType) {
        self.graphics = Some(graphics_type);
    }
    fn render(&mut self, buffer: &mut QuadBufferBuilder,sprite_verticies: &mut Vec<SpriteVertex>,
              sprite_indicies: &mut Vec<u16>,atlas: &Option<SpriteAtlas>) {
        match &self.graphics {
            Some(graphics) => match graphics {
                GraphicsType::Sprite(sprite) => {
                    if atlas.is_none() {
                        panic!("No atlas configured!"); //TODO: Proper error handling
                    }
                    let safe_atlas = atlas.as_ref().unwrap();
                    let sprite_data = safe_atlas.lookup_sprite_data_from_descriptor(&sprite.sprite_id);
                    let sprite = safe_atlas.get_sprite_from_atlas(&sprite_data.position,&sprite_data.sourceSize,[self.pos_x,self.pos_y],[sprite.width,sprite.height],sprite.flip_h,sprite.flip_v);
                    sprite_verticies.extend_from_slice(&sprite.0);
                    sprite_indicies.extend_from_slice(&sprite.1)
                }
                GraphicsType::Circle(circle) => {
                    buffer.push_circle(self.pos_x, self.pos_y, circle.radius, &circle.color, 60);
                }
                GraphicsType::Rect(square) => {
                    buffer.push_rect(
                        self.pos_x,
                        self.pos_y,
                        square.width,
                        square.height,
                        &square.color,
                    );
                }
                GraphicsType::Triangle(_triangle) => {
                    todo!()
                }
            },
            None => {}
        }
    }
}
