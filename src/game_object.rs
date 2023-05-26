use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::GetFrameTime;
use crate::game_object::behaviours::{UserBehaviour};
use crate::game_object::graphics::{Graphics, GraphicsType};
use crate::game_object::physics::PhysicsData;

pub mod physics;
pub mod graphics;
pub mod behaviours;


pub struct GameObject {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Box<dyn UserBehaviour + 'static>>,
    pub pos_x: i32,
    pub pos_y: i32,
    pub physics: Option<PhysicsData>
}

impl GameObject {
    pub fn new(pos_x: i32,pos_y: i32) -> Self {
        GameObject {
            graphics: None,
            behaviours: vec![],
            pos_y,
            pos_x,
            physics: None
        }
    }
    pub fn execute(&mut self,d: &mut RaylibDrawHandle) {
        let behaviours = &mut self.behaviours;

        let mut time: f32 = 0.0;
        unsafe {
            time = GetFrameTime();
        }

        for behaviour in behaviours {
            behaviour.game_loop(&mut self.pos_x,&mut self.pos_y,time);
        }
        self.render(d);
    }

}
