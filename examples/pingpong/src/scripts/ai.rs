use alcubierre::game_object::behaviours::{EngineView, UserBehaviour};
use alcubierre::game_object::{GameObject, GameObjectView};
use alcubierre::Engine;
use nalgebra::abs;
use rand::rngs::ThreadRng;
use rand::Rng;
use rapier2d::math::{Isometry, Real};
use rapier2d::prelude::{vector, ColliderHandle, RigidBodyHandle, Vector};

#[derive(Clone)]
pub struct AIBehaviour {
    pub(crate) speed: f32,
    pub(crate) ball_handle: ColliderHandle,
    pub(crate) ball_rigid_handle: RigidBodyHandle,
    pub(crate) rng: ThreadRng,
}

impl UserBehaviour for AIBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, engine_view: EngineView) {
        {
            if engine_view
                .is_colliding(
                    game_object_view.physics.collider_handle.unwrap(),
                    self.ball_handle,
                )
                .is_some()
            {
                let ball_rigid_body = engine_view
                    .rigid_body_set
                    .get_mut(self.ball_rigid_handle)
                    .unwrap();
                // ball_rigid_body.reset_forces(true);
                let x = self.rng.gen_range(0..100);
                if x > 50 {
                    ball_rigid_body.apply_impulse(vector![0.0, 0.00003], true);
                } else {
                    ball_rigid_body.apply_impulse(vector![0.0, -0.00003], true);
                }
            }
        }

        let mut ball_y: Real;
        let mut ball_vel: Vector<Real>;
        {
            let ball_rigid_body = engine_view
                .rigid_body_set
                .get_mut(self.ball_rigid_handle)
                .unwrap();
            ball_y = ball_rigid_body.translation().y;
            ball_vel = ball_rigid_body.linvel().clone();
        }

        let rigid_body = engine_view
            .rigid_body_set
            .get_mut(game_object_view.physics.rigid_body_handle.unwrap())
            .unwrap();

        let pos = rigid_body.position();

        let lower_edge = pos.translation.y - 150.0;
        let upper_edge = pos.translation.y + 2.0; // 2.0 is safety margin
        let y_range = upper_edge - lower_edge;
        // println!("YR: {}",y_range);

        if ball_y > lower_edge || ball_y > upper_edge && ball_vel.x < 0.0 {
            rigid_body.set_position(Isometry::new(vector![pos.translation.x, ball_y], 0.0), true);
        }
    }
}
