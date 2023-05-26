use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::GameObject;
use flame::keyboard::is_key_down;


pub struct PlayerBehaviour {
    pub(crate) speed: f32
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(&mut self, mut pos_x: &mut i32, pos_y: &mut i32,frame_delta: f32) {
        if is_key_down(KEY_RIGHT as i64) {
            *pos_x += (self.speed * frame_delta) as i32;
        }
        if is_key_down(KEY_LEFT as i64) {
            *pos_x -= (self.speed * frame_delta) as i32;
        }
        if is_key_down(KEY_DOWN as i64) {
            *pos_y += (self.speed * frame_delta) as i32;
        }
        if is_key_down(KEY_UP as i64) {
            *pos_y -= (self.speed * frame_delta) as i32;
        }
    }

    fn init(&mut self) {
        println!("INIT");
    }
}