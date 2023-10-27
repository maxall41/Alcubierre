use crate::game_object::GameObject;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::ui::frontend::RGBColor;
use std::f32::consts::PI;
use std::fs;
use std::fs::File;
use std::io::Read;
use cgmath::Rotation3;
use hashbrown::HashMap;
use crate::renderer::sprite::SpriteInstance;

pub type SpriteID = usize;

#[derive(Clone)]
pub struct SpriteData {
    pub sprite_id: SpriteID,
    pub filter: bool,
    pub width: f32,
    pub height: f32
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
    fn render(&mut self, buffer: &mut QuadBufferBuilder,sprite_instances: &mut Vec<SpriteInstance>);
}

pub(crate) fn get_file_as_byte_vector(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

impl Graphics for GameObject {
    fn add_graphics(&mut self, graphics_type: GraphicsType) {
        self.graphics = Some(graphics_type);
    }
    fn render(&mut self, buffer: &mut QuadBufferBuilder,sprite_instances: &mut Vec<SpriteInstance>) {
        match &self.graphics {
            Some(graphics) => match graphics {
                GraphicsType::Sprite(sprite) => {
                    sprite_instances.push(SpriteInstance {
                        position: cgmath::Vector3 { x: self.pos_x, y: self.pos_y, z: 0.0 },
                        rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
                        sprite_index: sprite.sprite_id as f32,
                    });
                }
                GraphicsType::Circle(circle) => {
                    // 1.55 here is to match physics scaling
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
