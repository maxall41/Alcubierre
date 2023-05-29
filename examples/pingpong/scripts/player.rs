use hashbrown::HashMap;
use nalgebra::abs;
use rapier2d::geometry::ColliderHandle;
use rapier2d::math::Isometry;
use rapier2d::prelude::{vector, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP};
use flame::FlameEngineView;
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::{is_key_down, is_key_pressed};

#[derive(Clone)]
pub struct PlayerBehaviour {
    pub(crate) speed: f32,
    pub(crate) decay: f32,
    pub(crate) ball_handle: ColliderHandle,
    pub(crate) score: i32
}

impl UserBehaviour for PlayerBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32) {
        let rigid_body = engine_view.rigid_body_set.get_mut(game_object_view.physics.rigid_body_handle.unwrap()).unwrap();
        let mut y_vel : f32 = 0.0;
        if is_key_down(KEY_UP as i64) {
            y_vel -= self.speed;
        }
        if is_key_down(KEY_DOWN as i64) {
            y_vel += self.speed;
        }

        let pos = rigid_body.position();
        rigid_body.set_position(Isometry::new(vector![pos.translation.x, pos.translation.y + y_vel], 0.0),true);

        if engine_view.is_colliding(game_object_view.physics.collider_handle.unwrap(), self.ball_handle) {
            self.score += 1;
            engine_view.set_datamap_value("ScoreValue".to_string(),self.score.to_string());
        }
    }

}