use alcubierre::game_object::behaviours::{EngineView, UserBehaviour};
use alcubierre::game_object::{GameObject, GameObjectView};
use alcubierre::Engine;
use nalgebra::abs;
use rand::rngs::ThreadRng;
use rand::Rng;
use rapier2d::math::{Isometry, Real};
use rapier2d::prelude::{vector, ColliderHandle, RigidBodyHandle, Vector};

#[derive(Clone)]
pub struct BarrierBehaviour {
    pub(crate) ball_handle: ColliderHandle,
    pub(crate) ball_rigid_handle: RigidBodyHandle,
    pub(crate) rng: ThreadRng,
}

impl UserBehaviour for BarrierBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, engine_view: EngineView) {
        // if engine_view.is_colliding(
        //     game_object_view.physics.collider_handle.unwrap(),
        //     self.ball_handle,
        // ) {
        //     let ball_rigid_body = engine_view
        //         .rigid_body_set
        //         .get_mut(self.ball_rigid_handle)
        //         .unwrap();
        //     // ball_rigid_body.reset_forces(true);
        //     let x = self.rng.gen_range(0..100);
        //     if x > 50 {
        //         ball_rigid_body.apply_impulse(vector![0.0, 0.00003], true);
        //     } else {
        //         ball_rigid_body.apply_impulse(vector![0.0, -0.00003], true);
        //     }
        // }
    }
}
