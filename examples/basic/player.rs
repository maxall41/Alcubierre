use rapier2d::prelude::{vector, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use flame::FlameEngineView;
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::is_key_down;


pub struct PlayerBehaviour {
    pub(crate) speed: f32
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32) {
        let rigid_body = engine_view.rigid_body_set.get_mut(game_object_view.physics.rigid_body_handle.unwrap()).unwrap();
        let mut x_vel : f32 = 0.0;
        let mut y_vel : f32 = 0.0;
        if is_key_down(KEY_RIGHT as i64) {
            // *view.pos_x += (self.speed * frame_delta) as i32;
            x_vel += self.speed;
        }
        if is_key_down(KEY_LEFT as i64) {
            // *view.pos_x  -= (self.speed * frame_delta) as i32;
            x_vel -= self.speed;
        }
        if is_key_down(KEY_DOWN as i64) {
            // *view.pos_y  += (self.speed * frame_delta) as i32;
            y_vel += self.speed;
        }
        if is_key_down(KEY_UP as i64) {
            // *view.pos_y -= (self.speed * frame_delta) as i32;
            y_vel -= self.speed;
        }

        let current_vel = rigid_body.linvel();

        rigid_body.set_linvel(Vector::new(current_vel.x + x_vel,current_vel.y + y_vel),false);
    }

    fn init(&mut self) {
        println!("INIT");
    }
}