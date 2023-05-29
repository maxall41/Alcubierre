use flame::game_object::behaviours::UserBehaviour;
use flame::game_object::{GameObject, GameObjectView};
use flame::keyboard::{is_key_down, is_key_pressed};
use flame::FlameEngineView;
use nalgebra::abs;
use rand::rngs::ThreadRng;
use rand::Rng;
use rapier2d::math::{Isometry, Real};
use rapier2d::prelude::{vector, ColliderHandle, RigidBodyHandle, Vector};
use raylib::ffi::GetFrameTime;
use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP};

#[derive(Clone)]
pub struct FailBehaviour {
    pub(crate) speed: f32,
    pub(crate) ball_handle: ColliderHandle,
}

impl UserBehaviour for FailBehaviour {
    fn game_loop(
        &mut self,
        game_object_view: GameObjectView,
        engine_view: FlameEngineView,
        frame_delta: f32,
    ) {
        if engine_view.is_colliding_with_sensor(
            game_object_view.physics.collider_handle.unwrap(),
            self.ball_handle,
        ) {
            engine_view.load_scene("Fail".to_string());
        }
    }
}
