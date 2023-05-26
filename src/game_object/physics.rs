use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::{Collider, ColliderHandle};
use crate::FlameEngine;
use crate::game_object::GameObject;

pub trait PhysicsObject {
    fn attach_collider(&mut self,collider: Collider,_: &mut FlameEngine);
    fn remove_collider(&mut self,_: &mut FlameEngine);
    fn attach_rigid_body(&mut self,rigid_body: RigidBody,_: &mut FlameEngine);
    fn remove_rigid_body(&mut self,_: &mut FlameEngine);
}

pub struct PhysicsData {
    collider: Option<ColliderHandle>
}

impl PhysicsObject for GameObject {
    fn attach_collider(&mut self, collider: Collider,flame: &mut FlameEngine) {
        let handle = flame.collider_set.insert(collider);
        self.physics = Some(PhysicsData {
            collider: Some(handle)
        });
    }

    fn remove_collider(&mut self,flame: &mut FlameEngine) {
        self.physics.as_mut().unwrap().collider = None;
    }

    fn attach_rigid_body(&mut self, rigid_body: RigidBody,flame: &mut FlameEngine) {
        todo!()
    }

    fn remove_rigid_body(&mut self,flame: &mut FlameEngine) {
        todo!()
    }
}