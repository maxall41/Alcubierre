use std::f32::consts::PI;
use crate::game_object::GameObject;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::ui::frontend::RGBColor;

#[derive(Clone)]
pub struct SpriteData {
    pub image: String,
    pub x_pos: f32,
    pub y_pos: f32,
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
    fn render(&mut self, buffer: &mut QuadBufferBuilder);
}

impl Graphics for GameObject {
    fn add_graphics(&mut self, graphics_type: GraphicsType) {
        self.graphics = Some(graphics_type);
    }
    fn render(&mut self, buffer: &mut QuadBufferBuilder) {
        match &self.graphics {
            Some(graphics) => match graphics {
                GraphicsType::Sprite(_sprite) => {
                    todo!()
                }
                GraphicsType::Circle(circle) => {
                    // 1.55 here is to match physics scaling
                    buffer.push_circle(self.pos_x, self.pos_y, circle.radius, &circle.color,60);
                }
                GraphicsType::Rect(square) => {
                    buffer.push_square(
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
