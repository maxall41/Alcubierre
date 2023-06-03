use alcubierre::game_object::behaviours::{EngineView, UserBehaviour};
use alcubierre::game_object::{GameObject, GameObjectView};
use nalgebra::abs;
use rand::rngs::ThreadRng;
use rand::Rng;
use rapier2d::math::{Isometry, Real};
use rapier2d::prelude::{vector, ColliderHandle, RigidBodyHandle, Vector};

#[derive(Clone)]
pub struct FailBehaviour {
    pub(crate) speed: f32,
    pub(crate) ball_handle: ColliderHandle,
}

impl UserBehaviour for FailBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, engine_view: EngineView) {
        if engine_view.is_colliding_with_sensor(
            game_object_view.physics.collider_handle.unwrap(),
            self.ball_handle,
        ) {
            engine_view.load_scene("Fail".to_string());
        }
    }
}
