use nalgebra::abs;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle, vector, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP};
use flame::FlameEngineView;
use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::{is_key_down, is_key_pressed};
use rand::Rng;
use rand::rngs::ThreadRng;
use rapier2d::math::{Isometry, Real};

#[derive(Clone)]
pub struct AIBehaviour {
    pub(crate) speed: f32,
    pub(crate) ball_handle: ColliderHandle,
    pub(crate) ball_rigid_handle: RigidBodyHandle,
    pub(crate) rng: ThreadRng
}

impl UserBehaviour for AIBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView,engine_view: FlameEngineView,frame_delta: f32) {
        {
            if engine_view.is_colliding(game_object_view.physics.collider_handle.unwrap(), self.ball_handle) {
                let ball_rigid_body = engine_view.rigid_body_set.get_mut(self.ball_rigid_handle).unwrap();
                ball_rigid_body.reset_forces(true);
                ball_rigid_body.apply_impulse(vector![0.0, self.rng.gen_range(-7.0..7.0)], true);
            }
        }

        let mut ball_y : Real;
        {
            let ball_rigid_body = engine_view.rigid_body_set.get_mut(self.ball_rigid_handle).unwrap();
            ball_y = ball_rigid_body.translation().y;
        }

        let rigid_body = engine_view.rigid_body_set.get_mut(game_object_view.physics.rigid_body_handle.unwrap()).unwrap();

        let pos = rigid_body.position();

        rigid_body.set_position(Isometry::new(vector![pos.translation.x, ball_y], 0.0),true);

    }

}