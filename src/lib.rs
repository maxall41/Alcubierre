pub mod game_object;
pub mod keyboard;

use std::thread::sleep;
use std::time::Duration;
use rapier2d::geometry::{ColliderBuilder, ColliderSet};
use raylib::{RaylibHandle, RaylibThread};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::ffi::{GetKeyPressed, IsKeyPressed};
use crate::game_object::GameObject;
use crate::game_object::graphics::Graphics;

pub struct FlameEngine {
    raylib: RaylibHandle,
    thread: RaylibThread,
    game_objects: Vec<GameObject>,
    collider_set: ColliderSet
}


impl FlameEngine {
    pub fn x() {

    }
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("Hello, World")
            .build();

        let collider_set = ColliderSet::new();


        FlameEngine {
            raylib: rl,
            thread: thread,
            game_objects: vec![],
            collider_set
        }
    }
    pub fn start_cycle(&mut self,game_code: fn(&mut Self)) {
        loop {
            { game_code(self); }
            let mut d = self.raylib.begin_drawing(&self.thread);

            for object in &mut self.game_objects {
                object.execute(&mut d);
            }

            d.clear_background(Color::WHITE);

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    pub fn insert_game_object(&mut self,game_object: GameObject) {
        self.game_objects.push(game_object);
    }
}
