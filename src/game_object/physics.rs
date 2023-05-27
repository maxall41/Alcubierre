use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::{Collider, ColliderHandle, ColliderSet};
use rapier2d::math::Real;
use rapier2d::prelude::{RigidBodyHandle, RigidBodySet, Vector};
use crate::FlameEngine;
use crate::game_object::GameObject;
use crate::physics::FlameCollider;
use crate::physics::physics_units_to_pixels;

pub trait PhysicsObject {
    fn attach_collider(&mut self,collider: FlameCollider,_: &mut FlameEngine) -> ColliderHandle;
    fn attach_collider_with_rigid_body(&mut self, collider: FlameCollider, flame: &mut FlameEngine) -> ColliderHandle;
    fn remove_collider(&mut self,_: &mut FlameEngine);
    fn attach_rigid_body(&mut self,rigid_body: RigidBody,_: &mut FlameEngine);
    fn remove_rigid_body(&mut self,_: &mut FlameEngine);
    fn get_updated_physics_position(&mut self, rigid_body_set: &mut RigidBodySet) -> (Real,Real);
}

#[derive(Clone)]
pub struct PhysicsData {
    pub collider_handle: Option<ColliderHandle>,
    pub rigid_body_handle: Option<RigidBodyHandle>
}

impl PhysicsObject for GameObject {
    fn attach_collider(&mut self, collider: FlameCollider,flame: &mut FlameEngine) -> ColliderHandle {
        let handle = flame.collider_set.insert(collider.to_rapier());
        self.physics.collider_handle = Some(handle.clone());
        handle
    }

    fn attach_collider_with_rigid_body(&mut self, collider: FlameCollider, flame: &mut FlameEngine) -> ColliderHandle {
        let handle = flame.collider_set.insert_with_parent(collider.to_rapier(),self.physics.rigid_body_handle.unwrap(),&mut flame.rigid_body_set);
        self.physics.collider_handle = Some(handle.clone());
        handle
    }

    fn remove_collider(&mut self,flame: &mut FlameEngine) {
        self.physics.collider_handle = None;
    }

    fn attach_rigid_body(&mut self, rigid_body: RigidBody,flame: &mut FlameEngine) {
        let handle = flame.rigid_body_set.insert(rigid_body);
        self.physics.rigid_body_handle = Some(handle)
    }

    fn remove_rigid_body(&mut self,flame: &mut FlameEngine) {
        todo!()
    }

    fn get_updated_physics_position(&mut self, rigid_body_set: &mut RigidBodySet) -> (Real,Real) {
        let collider = rigid_body_set.get(self.physics.rigid_body_handle.unwrap()).unwrap();
        let translation = collider.translation();
        return (physics_units_to_pixels(translation.x),physics_units_to_pixels(translation.y));
    }
}