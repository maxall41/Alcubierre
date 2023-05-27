use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::DrawCircle;
use crate::game_object::GameObject;

#[derive(Clone)]
pub struct SpriteData {
    pub image: String,
    pub x_pos: i32,
    pub y_pos: i32,
}
#[derive(Clone)]
pub struct CircleData {
    pub radius: f32,
    pub color: Color
}
#[derive(Clone)]
pub struct SquareData {
    pub color: Color,
    pub width: i32,
    pub height: i32
}
#[derive(Clone)]
pub struct TriangleData {
    pub radius: f32,
    pub color: Color
}
#[derive(Clone)]
pub enum GraphicsType {
    Sprite(SpriteData),
    Circle(CircleData),
    Square(SquareData),
    Triangle(TriangleData)
}

pub trait Graphics {
    fn add_graphics(&mut self,graphics_type: GraphicsType);
    fn render(&mut self,draw_handle: &mut RaylibDrawHandle<'_>);
}

impl Graphics for GameObject {
    fn add_graphics(&mut self,graphics_type: GraphicsType) {
        self.graphics = Some(graphics_type);
    }
    fn render(&mut self, mut draw_handle: &mut RaylibDrawHandle) {
        match &self.graphics {
            Some(graphics) => {
                match graphics {
                    GraphicsType::Sprite(sprite) => {
                        todo!()
                    }
                    GraphicsType::Circle(circle) => {
                        draw_handle.draw_circle(self.pos_x,self.pos_y,circle.radius,circle.color);
                    }
                    GraphicsType::Square(square) => {
                        draw_handle.draw_rectangle(self.pos_x,self.pos_y,square.width,square.height,square.color);
                    }
                    GraphicsType::Triangle(triangle) => {
                        todo!()
                    }
                }
            }
            None => {}
        }
    }

}