use crate::game_object::GameObject;
use crate::physics::physics_units_to_pixels;
use crate::physics::AlcubierreCollider;
use crate::Scene;
use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::ColliderHandle;
use rapier2d::math::Real;
use rapier2d::prelude::{RigidBodyHandle, RigidBodySet};

pub trait PhysicsObject {
    fn remove_collider(&mut self, scene: &mut Scene);
    fn remove_rigid_body(&mut self, scene: &mut Scene);
    fn get_updated_physics_position(&mut self, rigid_body_set: &mut RigidBodySet) -> (Real, Real);
}

#[derive(Clone)]
pub struct PhysicsData {
    pub collider_handle: Option<ColliderHandle>,
    pub rigid_body_handle: Option<RigidBodyHandle>,
}

impl PhysicsObject for GameObject {
    fn remove_collider(&mut self, scene: &mut Scene) {
        scene.collider_set.remove(
            self.physics.collider_handle.unwrap(),
            &mut scene.island_manager,
            &mut scene.rigid_body_set,
            true,
        );
        self.physics.collider_handle = None;
    }

    fn remove_rigid_body(&mut self, _scene: &mut Scene) {
        todo!()
    }

    fn get_updated_physics_position(&mut self, rigid_body_set: &mut RigidBodySet) -> (Real, Real) {
        let body = rigid_body_set
            .get(self.physics.rigid_body_handle.unwrap())
            .unwrap();
        let translation = body.translation();
        return (
            physics_units_to_pixels(translation.x),
            physics_units_to_pixels(translation.y),
        );
    }
}
